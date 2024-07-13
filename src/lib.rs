use hound::WavWriter;
use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn midi_to_wav(midi_data: &[u8], sf2_data: &[u8], sample_rate: u32, vol: f32) -> Vec<u8> {
    // サウンドフォントを読み込む
    let sound_font = std::sync::Arc::new(SoundFont::new(&mut &sf2_data[..]).unwrap());

    // シンセサイザーの設定と作成
    let settings = SynthesizerSettings::new(sample_rate as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    // MIDIシーケンサーの作成
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // MIDIファイルの読み込み
    let midi_file = std::sync::Arc::new(MidiFile::new(&mut &midi_data[..]).unwrap());
    let midi_time_len = midi_file.get_length();

    // 必要なサンプル数を計算
    let sample_count = (sample_rate as f64 * midi_time_len) as usize;

    // サンプルバッファを確保
    let mut samples = vec![0.0f32; sample_count * 2];
    let mut left_buf = vec![0.0f32; sample_count];
    let mut right_buf = vec![0.0f32; sample_count];

    // MIDIデータをレンダリング
    sequencer.play(&midi_file, false);
    sequencer.render(&mut left_buf[..], &mut right_buf[..]);

    for i in 0..left_buf.len() {
        samples[i * 2 + 0] = left_buf[i] * vol;
        samples[i * 2 + 1] = right_buf[i] * vol;
    }

    // WAVデータを作成
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut cursor = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut cursor, spec).unwrap();

    for sample in samples {
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();

    cursor.into_inner()
}
