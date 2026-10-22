//! `rustup++_init.exe`: the Rust++ installer for Windows.
//!
//! Installs the Rust++ toolchain (rustc, cargo, rustdoc, standard library) into
//! `%USERPROFILE%\.rustpp` and puts `%USERPROFILE%\.rustpp\bin` first in the
//! user's PATH, so `rustc` and `cargo` in any new terminal are Rust++. There is
//! no toolchain switching: Rust++ is the only toolchain it knows about.
//!
//! It deliberately has no dependencies: downloading, unzipping and editing the
//! PATH are done with tools that ship with Windows 10/11 (curl.exe, tar.exe and
//! PowerShell), so it builds with nothing but `rustc`.
//!
//! Usage:
//!   rustup++_init.exe                       install or update (interactive)
//!   rustup++_init.exe -y                    install without questions
//!   rustup++_init.exe --uninstall           remove Rust++
//!   rustup++_init.exe --from-file <zip>     install from a local zip

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

const REPO: &str = "AlzelemX/Rust-";
const ASSET: &str = "rustpp-x86_64-pc-windows-msvc.zip";
const BUILD_TOOLS_URL: &str = "https://visualstudio.microsoft.com/visual-cpp-build-tools/";

type Result<T> = std::result::Result<T, String>;

struct Options {
    yes: bool,
    uninstall: bool,
    from_file: Option<PathBuf>,
}

fn main() -> ExitCode {
    let opts = match parse_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!();
    println!("  Rust++ installer");
    println!("  ================");
    println!();

    let result = if opts.uninstall { uninstall(&opts) } else { install(&opts) };
    let code = match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!();
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    };
    // Keep the window open when the installer was started by double-clicking.
    if !opts.yes {
        prompt("\nPress Enter to close this window.");
    }
    code
}

fn parse_args() -> Result<Options> {
    let mut opts = Options { yes: false, uninstall: false, from_file: None };
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-y" | "--yes" => opts.yes = true,
            "--uninstall" => opts.uninstall = true,
            "--from-file" => {
                let path = args.next().ok_or("--from-file needs a path to a .zip")?;
                opts.from_file = Some(PathBuf::from(path));
            }
            "-h" | "--help" => {
                println!("rustup++_init.exe [-y] [--uninstall] [--from-file <zip>]");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument `{other}` (try --help)")),
        }
    }
    Ok(opts)
}

fn install_dir() -> Result<PathBuf> {
    let home = env::var_os("USERPROFILE").ok_or("USERPROFILE is not set")?;
    Ok(PathBuf::from(home).join(".rustpp"))
}

fn install(opts: &Options) -> Result<()> {
    let dir = install_dir()?;
    let bin = dir.join("bin");
    let updating = bin.join("rustc.exe").exists();

    println!("  This will {} Rust++ in:", if updating { "update" } else { "install" });
    println!("      {}", dir.display());
    println!("  and put this folder first in your PATH:");
    println!("      {}", bin.display());
    println!();
    println!("  After that, `rustc` and `cargo` in a new terminal are Rust++.");
    println!();
    if !opts.yes && !confirm("Continue?") {
        println!("Cancelled.");
        return Ok(());
    }

    let zip = match &opts.from_file {
        Some(path) => path.clone(),
        None => download()?,
    };
    step("Unpacking");
    unpack(&zip, &dir)?;
    if opts.from_file.is_none() {
        let _ = fs::remove_file(&zip);
    }

    step("Adding Rust++ to your PATH");
    set_user_path(&bin, true)?;

    step("Checking the C++ build tools (needed to link programs)");
    check_build_tools(opts);

    step("Checking the installation");
    for tool in ["rustc", "cargo"] {
        let exe = bin.join(format!("{tool}.exe"));
        let out = Command::new(&exe)
            .arg("--version")
            .output()
            .map_err(|e| format!("could not run {}: {e}", exe.display()))?;
        println!("      {}", String::from_utf8_lossy(&out.stdout).trim());
    }

    println!();
    println!("  Rust++ is installed.");
    println!();
    println!("  Open a NEW terminal (PowerShell or cmd) and try:");
    println!("      rustc --version");
    println!("      cargo new hello");
    println!("      cd hello");
    println!("      cargo run");
    println!();
    println!("  In Rust++ you can print with out!(\"...\") as well as println!(\"...\").");
    Ok(())
}

fn uninstall(opts: &Options) -> Result<()> {
    let dir = install_dir()?;
    println!("  This will remove Rust++ from:");
    println!("      {}", dir.display());
    println!();
    if !opts.yes && !confirm("Continue?") {
        println!("Cancelled.");
        return Ok(());
    }
    step("Removing Rust++ from your PATH");
    set_user_path(&dir.join("bin"), false)?;
    if dir.exists() {
        step("Deleting files");
        fs::remove_dir_all(&dir).map_err(|e| {
            format!("could not delete {}: {e}\n(close any program using Rust++ and retry)", dir.display())
        })?;
    }
    println!();
    println!("  Rust++ was removed. Open a new terminal for the PATH change to apply.");
    Ok(())
}

fn download() -> Result<PathBuf> {
    let url = format!("https://github.com/{REPO}/releases/latest/download/{ASSET}");
    let dest = env::temp_dir().join(ASSET);
    step("Downloading Rust++ (about 300 MB)");
    println!("      {url}");

    // curl.exe ships with Windows 10 (1803+) and 11 and shows a progress bar.
    let curl = Command::new("curl.exe")
        .args(["-fL", "--retry", "3", "--progress-bar", "-o"])
        .arg(&dest)
        .arg(&url)
        .status();
    if matches!(curl, Ok(s) if s.success()) {
        return Ok(dest);
    }

    println!("      curl.exe failed, retrying with PowerShell...");
    let script = format!(
        "$ProgressPreference = 'SilentlyContinue'; \
         Invoke-WebRequest -UseBasicParsing -Uri '{}' -OutFile '{}'",
        ps_quote(&url),
        ps_quote(&dest.to_string_lossy()),
    );
    powershell(&script).map_err(|e| format!("download failed: {e}"))?;
    Ok(dest)
}

/// Extracts the zip (which contains a top-level `rustpp` folder) into `dir`,
/// replacing any previous installation.
fn unpack(zip: &Path, dir: &Path) -> Result<()> {
    if !zip.is_file() {
        return Err(format!("{} does not exist", zip.display()));
    }
    let staging = dir.with_file_name(".rustpp-new");
    if staging.exists() {
        fs::remove_dir_all(&staging).map_err(|e| format!("cleaning {}: {e}", staging.display()))?;
    }
    fs::create_dir_all(&staging).map_err(|e| format!("creating {}: {e}", staging.display()))?;

    // tar.exe (bsdtar) ships with Windows 10/11 and can read zip files.
    let tar_ok = Command::new("tar.exe")
        .arg("-xf")
        .arg(zip)
        .arg("-C")
        .arg(&staging)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !tar_ok {
        let script = format!(
            "Expand-Archive -Force -LiteralPath '{}' -DestinationPath '{}'",
            ps_quote(&zip.to_string_lossy()),
            ps_quote(&staging.to_string_lossy()),
        );
        powershell(&script).map_err(|e| format!("could not unpack {}: {e}", zip.display()))?;
    }

    let unpacked = staging.join("rustpp");
    if !unpacked.join("bin").join("rustc.exe").exists() {
        return Err(format!("{} does not look like a Rust++ package", zip.display()));
    }
    if dir.exists() {
        fs::remove_dir_all(dir).map_err(|e| {
            format!(
                "could not replace {}: {e}\n(close any terminal or editor using Rust++ and retry)",
                dir.display()
            )
        })?;
    }
    fs::rename(&unpacked, dir).map_err(|e| format!("moving files into {}: {e}", dir.display()))?;
    let _ = fs::remove_dir_all(&staging);
    Ok(())
}

/// Adds `bin` to the front of the user PATH (or removes it).
///
/// The registry value is read without expanding `%VARIABLES%` and written back
/// as REG_EXPAND_SZ, so other PATH entries stay exactly as they were. Setting a
/// throwaway variable afterwards makes Windows broadcast the change, so new
/// terminals see the updated PATH without logging out.
fn set_user_path(bin: &Path, add: bool) -> Result<()> {
    let bin = ps_quote(&bin.to_string_lossy());
    let new_value = if add { format!("@('{bin}') + $parts") } else { "$parts".to_string() };
    let script = format!(
        "$key = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment'); \
         $p = $key.GetValue('Path', '', 'DoNotExpandEnvironmentNames'); \
         $parts = @($p -split ';' | Where-Object {{ $_ -ne '' -and $_.TrimEnd('\\') -ne '{bin}' }}); \
         $key.SetValue('Path', (({new_value}) -join ';'), 'ExpandString'); \
         $key.Close(); \
         [Environment]::SetEnvironmentVariable('RUSTPP_INSTALLER_REFRESH', '1', 'User'); \
         [Environment]::SetEnvironmentVariable('RUSTPP_INSTALLER_REFRESH', $null, 'User')"
    );
    powershell(&script).map_err(|e| format!("could not update PATH: {e}"))
}

/// Rust on Windows (MSVC) links programs with Microsoft's linker, which comes
/// with the Visual Studio C++ Build Tools. Look for it with vswhere.
fn check_build_tools(opts: &Options) {
    if build_tools_installed() {
        println!("      found");
        return;
    }
    println!();
    println!("      The Visual Studio C++ Build Tools were not found.");
    println!("      Rust++ needs them to turn your programs into .exe files.");
    println!();
    let can_winget = Command::new("winget")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if can_winget && !opts.yes && confirm("Install them now with winget (a few GB, takes a while)?") {
        let status = Command::new("winget")
            .args([
                "install",
                "--id",
                "Microsoft.VisualStudio.2022.BuildTools",
                "-e",
                "--source",
                "winget",
                "--accept-package-agreements",
                "--accept-source-agreements",
                "--override",
                "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended",
            ])
            .status();
        if matches!(status, Ok(s) if s.success()) {
            println!("      Build Tools installed.");
            return;
        }
        println!("      winget could not install them.");
    }
    println!("      Install \"Desktop development with C++\" from:");
    println!("          {BUILD_TOOLS_URL}");
    println!("      then Rust++ is ready to build programs.");
}

fn build_tools_installed() -> bool {
    let Some(pf) = env::var_os("ProgramFiles(x86)") else { return false };
    let vswhere = PathBuf::from(pf).join(r"Microsoft Visual Studio\Installer\vswhere.exe");
    if !vswhere.exists() {
        return false;
    }
    Command::new(vswhere)
        .args([
            "-latest",
            "-products",
            "*",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            "-property",
            "installationPath",
        ])
        .output()
        .map(|o| o.status.success() && !String::from_utf8_lossy(&o.stdout).trim().is_empty())
        .unwrap_or(false)
}

fn powershell(script: &str) -> Result<()> {
    let out = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", script])
        .output()
        .map_err(|e| format!("could not start PowerShell: {e}"))?;
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

/// Escapes a value for use inside a single-quoted PowerShell string.
fn ps_quote(s: &str) -> String {
    s.replace('\'', "''")
}

fn step(msg: &str) {
    println!("  > {msg}...");
}

fn prompt(msg: &str) -> String {
    print!("{msg} ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    let _ = io::stdin().lock().read_line(&mut line);
    line.trim().to_string()
}

fn confirm(question: &str) -> bool {
    let answer = prompt(&format!("  {question} [Y/n]")).to_lowercase();
    answer.is_empty() || answer == "y" || answer == "yes"
}
