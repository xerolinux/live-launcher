use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use libadwaita as adw;

const APP_ID: &str = "org.xerolinux.welcome";

// For testing, use local paths; for production, use system paths
#[cfg(debug_assertions)]
const LOGO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/xero.png");
#[cfg(debug_assertions)]
const ICONS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

#[cfg(not(debug_assertions))]
const LOGO_PATH: &str = "/usr/share/live-launcher/xero.png";
#[cfg(not(debug_assertions))]
const ICONS_PATH: &str = "/usr/share/live-launcher/icons";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| {
        load_css();
    });

    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(
        r#"
        .welcome-window {
            background: linear-gradient(180deg, #1a1535 0%, #2d2050 40%, #3d2a55 70%, #4a2d60 100%);
        }
        headerbar {
            background: transparent;
            box-shadow: none;
            border: none;
        }
        .logo-container {
            margin-top: 24px;
            margin-bottom: 16px;
        }
        @keyframes logo-glow {
            0%, 100% {
                filter: drop-shadow(0 0 8px rgba(180, 120, 255, 0.4)) drop-shadow(0 0 20px rgba(140, 80, 220, 0.3));
            }
            50% {
                filter: drop-shadow(0 0 15px rgba(200, 150, 255, 0.7)) drop-shadow(0 0 35px rgba(160, 100, 240, 0.5));
            }
        }
        .logo-glow {
            animation: logo-glow 3s ease-in-out infinite;
        }
        @keyframes star-glow-1 {
            0%, 100% { opacity: 0.2; }
            50% { opacity: 1.0; }
        }
        @keyframes star-glow-2 {
            0%, 100% { opacity: 0.8; }
            50% { opacity: 0.2; }
        }
        @keyframes star-glow-3 {
            0%, 100% { opacity: 0.5; }
            25% { opacity: 1.0; }
            75% { opacity: 0.1; }
        }
        .star {
            background: radial-gradient(circle, rgba(255,255,255,0.9) 0%, rgba(200,180,255,0.5) 40%, transparent 70%);
            border-radius: 50%;
            box-shadow: 0 0 6px 2px rgba(200, 180, 255, 0.5);
        }
        .star-1 {
            animation: star-glow-1 3s ease-in-out infinite;
        }
        .star-2 {
            animation: star-glow-2 4s ease-in-out infinite;
        }
        .star-3 {
            animation: star-glow-3 5s ease-in-out infinite;
        }
        .title-label {
            font-size: 28px;
            font-weight: bold;
            color: #ffffff;
            margin-bottom: 8px;
        }
        .description-label {
            font-size: 14px;
            color: #b8b8b8;
            margin-bottom: 16px;
        }
.install-button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            font-size: 16px;
            font-weight: bold;
            padding: 14px 48px;
            border-radius: 8px;
            margin: 16px;
            min-height: 48px;
        }
        .install-button:hover {
            background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
        }
        .separator-line {
            background-color: #4a4a6a;
            min-height: 1px;
            margin: 16px 32px;
        }
        .links-label {
            font-size: 13px;
            color: #9090a0;
            margin-bottom: 8px;
        }
        .link-button {
            background-color: rgba(60, 50, 90, 0.7);
            border-radius: 8px;
            padding: 8px 16px;
            margin: 4px;
            min-width: 100px;
        }
        .link-button:hover {
            background-color: rgba(80, 70, 120, 0.8);
        }
        .link-button-label {
            color: #ffffff;
            font-size: 12px;
        }
        .links-container {
            margin: 8px 16px 24px 16px;
        }
        "#,
    );

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    // Create overlay for stars background
    let overlay = gtk::Overlay::new();

    // Stars background layer
    let stars_container = gtk::Fixed::new();
    stars_container.set_hexpand(true);
    stars_container.set_vexpand(true);

    // Create animated stars at various positions
    let star_positions: Vec<(f64, f64, i32, i32)> = vec![
        // (x%, y%, size, animation_class 1-3)
        (5.0, 8.0, 3, 1), (15.0, 15.0, 2, 2), (25.0, 5.0, 4, 3),
        (35.0, 20.0, 2, 1), (45.0, 10.0, 3, 2), (55.0, 18.0, 2, 3),
        (65.0, 6.0, 4, 1), (75.0, 14.0, 3, 2), (85.0, 8.0, 2, 3),
        (95.0, 12.0, 3, 1), (10.0, 85.0, 2, 2), (20.0, 90.0, 3, 3),
        (30.0, 82.0, 4, 1), (40.0, 92.0, 2, 2), (50.0, 88.0, 3, 3),
        (60.0, 95.0, 2, 1), (70.0, 84.0, 4, 2), (80.0, 91.0, 3, 3),
        (90.0, 86.0, 2, 1), (8.0, 45.0, 3, 2), (92.0, 50.0, 2, 3),
        (3.0, 65.0, 4, 1), (97.0, 35.0, 3, 2), (12.0, 30.0, 2, 3),
        (88.0, 70.0, 3, 1), (6.0, 55.0, 2, 2), (94.0, 60.0, 4, 3),
        (18.0, 75.0, 3, 1), (82.0, 25.0, 2, 2), (4.0, 40.0, 3, 3),
        (96.0, 45.0, 2, 1), (22.0, 60.0, 4, 2), (78.0, 78.0, 3, 3),
    ];

    overlay.set_child(Some(&stars_container));

    // Main content
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.set_halign(gtk::Align::Center);
    content.set_valign(gtk::Align::Center);

    // Logo - centered with glow
    let logo_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    logo_container.add_css_class("logo-container");

    let logo = gtk::Image::from_file(LOGO_PATH);
    logo.set_pixel_size(148);
    logo.add_css_class("logo-glow");
    logo_container.append(&logo);
    content.append(&logo_container);

    // Title - centered
    let title = gtk::Label::new(Some("Welcome to XeroLinux"));
    title.add_css_class("title-label");
    content.append(&title);

    // Description - centered
    let description = gtk::Label::new(Some(
        "Experience a beautiful Arch-based distribution designed for simplicity, and performance.\nCustomized with care to provide you with the ultimate Linux desktop experience."
    ));
    description.add_css_class("description-label");
    description.set_justify(gtk::Justification::Center);
    description.set_wrap(true);
    description.set_max_width_chars(80);
    content.append(&description);

    // Install Button - centered
    let buttons_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    buttons_box.set_halign(gtk::Align::Center);

    let install_button = gtk::Button::with_label("Launch XeroLinux Installer");
    install_button.add_css_class("install-button");
    install_button.connect_clicked(|_| {
        let _ = std::process::Command::new("sh")
            .args(["-c", "sudo xen"])
            .spawn();
    });
    buttons_box.append(&install_button);

    content.append(&buttons_box);

    // Separator
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    separator.add_css_class("separator-line");
    content.append(&separator);

    // Links Section Label
    let links_label = gtk::Label::new(Some("Connect with the Community"));
    links_label.add_css_class("links-label");
    content.append(&links_label);

    // Links - Row 1 (3 buttons) - centered
    let row1 = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    row1.set_halign(gtk::Align::Center);
    row1.set_margin_top(8);

    let links_row1: Vec<(&str, &str, &str)> = vec![
        ("Website", "website.svg", "https://xerolinux.xyz"),
        ("Discord", "discord.svg", "https://discord.xerolinux.xyz"),
        ("YouTube", "youtube.svg", "https://youtube.com/@XeroLinux"),
    ];

    for (name, icon, url) in links_row1 {
        let button = create_link_button(name, icon, url);
        row1.append(&button);
    }
    content.append(&row1);

    // Links - Row 2 (2 buttons) - centered
    let row2 = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    row2.set_halign(gtk::Align::Center);
    row2.set_margin_top(8);
    row2.set_margin_bottom(20);

    let links_row2: Vec<(&str, &str, &str)> = vec![
        ("GitHub", "github.svg", "https://github.com/xerolinuxdev"),
        ("Donate", "donate.svg", "https://ko-fi.com/XeroLinux"),
    ];

    for (name, icon, url) in links_row2 {
        let button = create_link_button(name, icon, url);
        row2.append(&button);
    }
    content.append(&row2);

    overlay.add_overlay(&content);

    // Header bar with close and minimize buttons
    let header = gtk::HeaderBar::new();
    header.set_show_title_buttons(true);
    header.set_decoration_layout(Some(":minimize,close"));

    // Window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("XeroLinux Live Welcome")
        .default_width(900)
        .default_height(720)
        .resizable(false)
        .child(&overlay)
        .build();

    // Add stars after window is realized to get proper dimensions
    window.connect_realize(move |win| {
        let width = win.default_width() as f64;
        let height = win.default_height() as f64;

        for (x_pct, y_pct, size, anim) in &star_positions {
            let star = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            star.set_size_request(*size, *size);
            star.add_css_class("star");
            star.add_css_class(&format!("star-{}", anim));

            let x = (x_pct / 100.0 * width) as i32;
            let y = (y_pct / 100.0 * height) as i32;
            stars_container.put(&star, x as f64, y as f64);
        }
    });

    window.set_titlebar(Some(&header));
    window.add_css_class("welcome-window");
    window.present();
}

fn create_link_button(name: &str, icon_file: &str, url: &str) -> gtk::Button {
    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    button_box.set_halign(gtk::Align::Center);

    let icon_path = format!("{}/{}", ICONS_PATH, icon_file);
    let icon = if std::path::Path::new(&icon_path).exists() {
        gtk::Image::from_file(&icon_path)
    } else {
        gtk::Image::from_icon_name("web-browser-symbolic")
    };
    icon.set_pixel_size(20);
    button_box.append(&icon);

    let label = gtk::Label::new(Some(name));
    label.add_css_class("link-button-label");
    button_box.append(&label);

    let button = gtk::Button::new();
    button.set_child(Some(&button_box));
    button.add_css_class("link-button");

    let url = url.to_string();
    button.connect_clicked(move |_| {
        let _ = gio::AppInfo::launch_default_for_uri(&url, None::<&gio::AppLaunchContext>);
    });

    button
}
