use std::f32::consts::PI;

slint::include_modules!();
fn _calculate_wavepropagation() {
    // // Tuning audio for sound at a certain frequency for a listener a certain distance from the speakers
    // // Speaker Sources:
    // // Frequency: $ f = 440 Hz$
    // let compute_buffer: Vec<Vec<f32>> = vec![vec![0.0; 256]; 256];
    // let unit_distance = 1.0f32; // cm/unit
    // let sound_sources = vec![vec![128-8, 32], vec![128+8, 32]]; // two devices centered at (128, 32)
}

fn main() {
    let _ = dotenvy::dotenv();

    let host;
    #[cfg(target_os = "windows")]
    {
        host = cpal::host_from_id(cpal::HostId::Asio).expect("failed to initialise ASIO host");
    }
    let f_sampling: u32 = 44_100; // CD frequency
    let s_lut: Vec<f32> = (0..f_sampling)
        .map(|x| ((x as f32) * 2.0 * PI / (f_sampling as f32)).sin())
        .collect();


    // LPFWindow::new().unwrap().run().unwrap();
    // Keyboard::new().unwrap().run().unwrap();
    // Dials::new().unwrap().run().unwrap();
}
