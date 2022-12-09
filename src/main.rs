use clap::{Parser, command};

pub mod image;
pub mod mount;
pub mod unmount;
pub mod chroot;
pub mod shell;
pub mod unshare;
pub mod namespace;

#[derive(Parser)]
#[clap(name = "Rocker")]
#[clap(version = "0.1")]
#[clap(author = "Aditya Narayan Sinha (0x0elliot) <adityanrsinha@gmail.com>")]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Image,
    Mount,
    Unmount,
    Chroot,  
    Shell,
    Unshare,
    Namespace,
}

fn main() {
    let cli = Cli::parse();
    match cli.action {
        Action::Image => image::handle_image_command(),
        Action::Mount => mount::handle_mount_command(),
        Action::Unmount => unmount::handle_unmount_command(),
        Action::Chroot => chroot::handle_chroot_command(),
        Action::Shell => shell::handle_shell_command(),
        Action::Unshare => unshare::handle_unshare_command(),
        Action::Namespace => namespace::handle_namespace_command(),
    }    
}
