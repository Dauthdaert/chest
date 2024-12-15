use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cmd {
    #[arg(value_enum)]
    shell: Shell,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Shell {
    Nu,
    Zsh,
    Powershell,
}

impl Cmd {
    fn init_nu(&self) {
        let full = include_str!("../shell/chest.nu");
        println!("{full}");
    }

    fn init_zsh(&self) {
        let full = include_str!("../shell/chest.zsh");
        println!("{full}");
    }

    fn init_ps(self) {
        let full = include_str!("../shell/chest.ps1");
        println!("{full}");
    }

    pub fn run(self) {
        match self.shell {
            Shell::Nu => self.init_nu(),
            Shell::Zsh => self.init_zsh(),
            Shell::Powershell => self.init_ps(),
        }
    }
}
