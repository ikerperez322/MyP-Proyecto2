use std::{io::BufReader};
use std::fs::File;

use rodio::{Decoder, OutputStream, Sink};

pub struct Reproductor {
    sink: Sink,
    _stream: OutputStream,
}

impl Reproductor {

    //Método constructor para el Reproductor
    pub fn new() -> Self {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        
        return Self { sink: sink,
            _stream: _stream,
        };
    }

    //Método para reproducir una canción a partir del path recibido
    pub fn reproduce_cancion(&self, path: &str) -> Result<(), Box<dyn::std::error::Error>> {
        
        let cancion = File::open(path)?;
        let source = Decoder::new(BufReader::new(cancion))?;

        self.sink.append(source);
        
        return Ok(());
    }    
}

