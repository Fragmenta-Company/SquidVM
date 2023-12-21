use clap::Parser;

/// Argument Parser
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// Binary File Input | Don't need extension (.sqdbin)
    #[arg(short, long, value_name = "FILE", conflicts_with = "sar")]
    pub bin: Option<String>,

    /// Squid ARchive File Input | Don't need extension (.sar)
    #[arg(short, long, value_name = "FILE", conflicts_with = "bin")]
    pub sar: Option<String>,

    /// Max Memory Allocated for the heap | Postfixes: GB, MB, KB, B
    #[arg(short, long, value_name = "SIZE", default_value = "512MB")]
    pub maxmem: String,

    /// The repository is used for saving global variables easily
    #[arg(short, long, value_name = "SIZE", default_value = "20")]
    pub repo_size: usize,

    /// Check the binary version for binaries with metadata included | Works with bin or sar
    #[arg(
        long,
        value_name = "FILE",
        requires = "bin",
        requires = "sar",
        short = 'v'
    )]
    pub binver: bool,

    /// Force binaries compiled for newer versions to be run.
    #[arg(long, requires = "bin", requires = "sar")]
    pub force_newer_bin: bool,

    /// Shows the SquidVM version | SquidVM |major|.|minor|.|patch|-|details| for |OS| |arch|
    #[arg(long, short = 'V')]
    pub version: bool,
}

/// Converts strings with postfixes (GB, MB, KB or B) into a value in bytes
pub fn string_to_bytesize(string: String) -> Result<usize, &'static str> {
    // dev_print!("Before pop: {}", string);

    if string.ends_with("GB") {
        let mut gb = string;

        gb.pop().unwrap();
        gb.pop().unwrap();

        // dev_print!("After pop: {}", gb);

        if let Ok(gb) = gb.parse::<f64>() {
            Ok((gb * (1024 * 1024 * 1024) as f64) as usize)
        } else {
            Err("Failed to parse numeric part")
        }
    } else if string.ends_with("MB") {
        let mut mb = string;

        mb.pop().unwrap();
        mb.pop().unwrap();

        // dev_print!("After pop: {}", mb);

        if let Ok(mb) = mb.parse::<f64>() {
            Ok((mb * (1024 * 1024) as f64) as usize)
        } else {
            Err("Failed to parse numeric part")
        }
    } else if string.ends_with("KB") {
        let mut kb = string;

        kb.pop().unwrap();
        kb.pop().unwrap();

        // dev_print!("After pop: {}", kb);

        if let Ok(kb) = kb.parse::<f64>() {
            Ok((kb * 1024f64) as usize)
        } else {
            Err("Failed to parse numeric part")
        }
    } else if string.ends_with("B") {
        let mut b = string;

        b.pop().unwrap();

        // dev_print!("After pop: {}", b);

        if let Ok(b) = b.parse::<usize>() {
            Ok(b)
        } else {
            Err("Failed to parse numeric part")
        }
    } else {
        Err("Number need a postfix -> GB, MB, KB or B")
    }
}
