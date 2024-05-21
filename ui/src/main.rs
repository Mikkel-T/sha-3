#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::Clipboard;
use floem::{
    action::open_file,
    cosmic_text::Weight,
    file::FileDialogOptions,
    kurbo::Size,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, RwSignal},
    views::{button, dyn_container, empty, h_stack, label, text, text_input, v_stack, Decorators},
    window::WindowConfig,
    Application, IntoView, View,
};
use sha3::{sha3, shake, Input};
use std::fs;

#[derive(Clone)]
enum InputMethod {
    File,
    Text,
}

#[derive(Clone, Debug)]
enum Algorithm {
    SHA3(SHA3Variant),
    SHAKE(SHAKEVariant),
}

#[derive(Clone, Debug)]
enum SHA3Variant {
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
}

#[derive(Clone, Debug)]
enum SHAKEVariant {
    SHAKE128,
    SHAKE256,
}

fn algorithm_to_string(algorithm: Algorithm) -> String {
    match algorithm {
        Algorithm::SHA3(variant) => match variant {
            SHA3Variant::SHA3_224 => "SHA3-224",
            SHA3Variant::SHA3_256 => "SHA3-256",
            SHA3Variant::SHA3_384 => "SHA3-384",
            SHA3Variant::SHA3_512 => "SHA3-512",
        },
        Algorithm::SHAKE(variant) => match variant {
            SHAKEVariant::SHAKE128 => "SHAKE128",
            SHAKEVariant::SHAKE256 => "SHAKE256",
        },
    }
    .to_string()
}

fn run_algorithm<T: Input>(algorithm: Algorithm, input: T, size: usize) -> String {
    match algorithm {
        Algorithm::SHA3(variant) => match variant {
            SHA3Variant::SHA3_224 => sha3(224, input),
            SHA3Variant::SHA3_256 => sha3(256, input),
            SHA3Variant::SHA3_384 => sha3(384, input),
            SHA3Variant::SHA3_512 => sha3(512, input),
        },
        Algorithm::SHAKE(variant) => match variant {
            SHAKEVariant::SHAKE128 => shake(128, input, size),
            SHAKEVariant::SHAKE256 => shake(256, input, size),
        },
    }
}

fn text_user_input(
    input_hash: RwSignal<String>,
    algorithm: RwSignal<Algorithm>,
    size: RwSignal<usize>,
) -> impl IntoView {
    let input = create_rw_signal(String::new());

    create_effect(move |_| {
        input_hash.set(run_algorithm(algorithm.get(), input.get(), size.get()));
    });

    v_stack((
        label(|| "Indsæt tekst"),
        text_input(input)
            .keyboard_navigatable()
            .placeholder("Input")
            .style(|s| s.width_full()),
    ))
    .style(|s| s.width_full().items_center().justify_center())
}

fn file_user_input(
    input_hash: RwSignal<String>,
    algorithm: RwSignal<Algorithm>,
    size: RwSignal<usize>,
) -> impl IntoView {
    let selected_file = create_rw_signal(String::new());
    let file_contents = create_rw_signal(Vec::new());

    create_effect(move |_| {
        input_hash.set(run_algorithm(
            algorithm.get(),
            file_contents.get(),
            size.get(),
        ));
    });

    v_stack((h_stack((
        button(|| "Vælg fil").on_click_cont(move |_| {
            open_file(
                FileDialogOptions::new().title("Vælg fil"),
                move |file_info| {
                    if let Some(file) = file_info {
                        selected_file.set(file.path[0].clone().to_string_lossy().to_string());
                        if let Ok(contents) = fs::read(file.path[0].clone()) {
                            file_contents.set(contents)
                        }
                    }
                },
            );
        }),
        label(move || {
            if selected_file.get() != String::new() {
                selected_file.get()
            } else {
                "Ingen fil valgt endnu".to_string()
            }
        }),
    ))
    .style(|s| s.gap(10., 0.).items_center().justify_center()),))
    .style(|s| s.width_full().items_center().justify_center())
}

fn app_view() -> impl View {
    let input_hash = create_rw_signal(String::new());
    let algorithm = create_rw_signal(Algorithm::SHA3(SHA3Variant::SHA3_256));
    let size = create_rw_signal(256);
    let size_str = create_rw_signal("256".to_string());
    let current = create_rw_signal(InputMethod::Text);

    create_effect(move |_| {
        let tmp = size_str.get().parse().unwrap_or(256);
        size.set(tmp - tmp % 8);
    });

    v_stack((
        text("SHA-3").style(|s| s.font_size(35.).font_weight(Weight::BOLD)),
        text("Vælg algoritme").style(|s| s.font_weight(Weight::SEMIBOLD)),
        h_stack((
            button(|| "SHA-3")
                .on_click_stop(move |_| algorithm.set(Algorithm::SHA3(SHA3Variant::SHA3_256))),
            button(|| "SHAKE")
                .on_click_stop(move |_| algorithm.set(Algorithm::SHAKE(SHAKEVariant::SHAKE256))),
        ))
        .style(|s| s.gap(10., 0.)),
        text("Vælg kapacitet").style(|s| s.font_weight(Weight::SEMIBOLD)),
        dyn_container(move || match algorithm.get() {
            Algorithm::SHA3(_) => h_stack((
                button(|| "224")
                    .on_click_stop(move |_| algorithm.set(Algorithm::SHA3(SHA3Variant::SHA3_224))),
                button(|| "256")
                    .on_click_stop(move |_| algorithm.set(Algorithm::SHA3(SHA3Variant::SHA3_256))),
                button(|| "384")
                    .on_click_stop(move |_| algorithm.set(Algorithm::SHA3(SHA3Variant::SHA3_384))),
                button(|| "512")
                    .on_click_stop(move |_| algorithm.set(Algorithm::SHA3(SHA3Variant::SHA3_512))),
            ))
            .style(|s| s.gap(10., 0.))
            .into_any(),

            Algorithm::SHAKE(_) => v_stack((
                h_stack((
                    button(|| "128").on_click_stop(move |_| {
                        algorithm.set(Algorithm::SHAKE(SHAKEVariant::SHAKE128))
                    }),
                    button(|| "256").on_click_stop(move |_| {
                        algorithm.set(Algorithm::SHAKE(SHAKEVariant::SHAKE256))
                    }),
                ))
                .style(|s| s.gap(10., 0.)),
                text("Vælg outputlængde").style(|s| s.font_weight(Weight::SEMIBOLD)),
                text_input(size_str)
                    .keyboard_navigatable()
                    .placeholder("Outputlængde"),
            ))
            .style(|s| s.gap(0., 10.).items_center().justify_center())
            .into_any(),
        }),
        label(move || format!("Algoritme: {}", algorithm_to_string(algorithm.get()))),
        text("Vælg inputmetode").style(|s| s.font_weight(Weight::SEMIBOLD)),
        h_stack((
            button(|| "Tekst").on_click_stop(move |_| current.set(InputMethod::Text)),
            button(|| "Fil").on_click_stop(move |_| {
                current.set(InputMethod::File);
                input_hash.set(run_algorithm(algorithm.get(), "", size.get()));
            }),
        ))
        .style(|s| s.gap(10., 0.)),
        dyn_container(move || match current.get() {
            InputMethod::File => file_user_input(input_hash, algorithm, size).into_any(),
            InputMethod::Text => text_user_input(input_hash, algorithm, size).into_any(),
        })
        .style(|s| s.width_full()),
        empty().style(|s| s.width_full().border(1.).border_color(Color::LIGHT_GRAY)),
        text("Hash:").style(|s| s.font_size(15.).font_weight(Weight::SEMIBOLD)),
        h_stack((
            label(move || input_hash.get()).style(|s| {
                s.padding(5.)
                    .border(1.)
                    .border_color(Color::GRAY)
                    .border_radius(2.)
            }),
            button(|| "Kopiér").on_click_stop(move |_| {
                let mut clipboard = Clipboard::new().unwrap();

                clipboard.set_text(input_hash.get()).unwrap();
            }),
        ))
        .style(|s| s.gap(10., 0.).justify_center().items_center()),
    ))
    .style(|s| {
        s.items_center()
            .justify_center()
            .width_full()
            .height_full()
            .gap(0., 10.)
            .padding(30.)
            .font_size(16.)
    })
}

fn main() {
    Application::new()
        .window(
            |_| app_view(),
            Some(
                WindowConfig::default()
                    .size(Size::new(1200., 600.))
                    .title("SHA-3"),
            ),
        )
        .run();
}
