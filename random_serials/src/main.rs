use rand::Rng;
use rfd::FileDialog;
use std::{fs::File, io::Write};

struct Maschine {
    herstellungsjahr: u8,
    kalenderwoche: u8,
    seriennummer: u32,
    typ: String,
}

impl Maschine {
    fn get_serial(&self) -> String {
        format!(
            "{:?}-{:?}-{:?}",
            self.herstellungsjahr, self.kalenderwoche, self.seriennummer
        )
    }
}

fn main() {
    let types = ["Dosierer", "Fasspumpe", "Fasspumpe", "Materialverteiler"];

    let mut file_output: Vec<String> = Vec::new();

    {
        let mut maschinen_liste: Vec<Maschine> = Vec::new();

        let mut rng = rand::thread_rng();

        for _ in 0..50 {
            for item in types {
                let jahr = rng.gen_range(21..=23);
                let woche = rng.gen_range(1..=52);
                let sn = rng.gen_range(0..=999999);

                maschinen_liste.push(Maschine {
                    herstellungsjahr: jahr,
                    kalenderwoche: woche,
                    seriennummer: sn,
                    typ: item.to_string(),
                })
            }
        }

        file_output.push(String::from("serial;type;"));

        for item in maschinen_liste {
            file_output.push(format!("{:?};{:?};", &item.get_serial(), &item.typ));
        }
    }

    let pathbuf = FileDialog::new()
        .add_filter("Tabelle", &["csv"])
        .set_file_name("output")
        .set_title("Speichern...")
        .pick_folder()
        .expect("Failed to open file dialog.");

    let mut file = File::create(pathbuf.join("output.csv")).expect("Failed to open file.");

    for item in file_output {
        writeln!(file, "{}", item).expect("Error to write line in file.")
    }
    //Tabelle erstellen
    //als csv speichern
}
