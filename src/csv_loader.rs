use {
    csv::{
        ReaderBuilder
    },
    std::{
        io,
    },
    encoding::{
        all::UTF_8,
        DecoderTrap,
        Encoding,
        RawDecoder,
        StringWriter,
    },
};

fn decode_byte_stream_to_string(
    _decoder: &mut dyn RawDecoder,
    input: &[u8],
    output: &mut dyn StringWriter,
) -> bool {
    for a in input {
        match *a {
            128 => output.write_char('C'),
            129 => output.write_char('u'),
            130 => output.write_char('e'),
            131 => output.write_char('a'),
            132 => output.write_char('a'),
            133 => output.write_char('a'),
            134 => output.write_char('a'),
            135 => output.write_char('c'),
            136 => output.write_char('e'),
            137 => output.write_char('e'),
            138 => output.write_char('e'),
            139 => output.write_char('i'),
            140 => output.write_char('i'),
            141 => output.write_char('i'),
            142 => output.write_char('A'),
            143 => output.write_char('A'),
            144 => output.write_char('E'),
            145 => output.write_char('a'),
            146 => output.write_char('\''),
            147 => output.write_char('o'),
            148 => output.write_char('o'),
            149 => output.write_char('o'),
            150 => output.write_char('u'),
            151 => output.write_char('u'),
            152 => output.write_char('y'),
            153 => output.write_char('O'),
            154 => output.write_char('U'),
            155 => output.write_char('o'),
            201 => output.write_char('E'),
            203 => output.write_char('E'),
            208 => output.write_char('D'),
            209 => output.write_char('N'),
            210 => output.write_char('E'),
            211 => output.write_char('O'),
            212 => output.write_char('E'),
            241 => output.write_char('n'),
            _char => println!("char: {:?}", _char),
        }
    };
    true
}

static DECODE_BYTES_TO_STRING: DecoderTrap = DecoderTrap::Call(decode_byte_stream_to_string);

#[derive(Debug)]
pub struct CsvLoader {
    path: String,
}

impl CsvLoader {
    pub fn from_path(path: &String) -> io::Result<Vec<Vec<String>>> {
        //let file = File::open(path)?;
        let rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path);

        let mut raw_table: Vec<Vec<String>> = Vec::new();
        for res in rdr?.byte_records() {
            let row = res?;
            raw_table.push(row.iter().map(|cols|
                UTF_8.decode(cols, DECODE_BYTES_TO_STRING).unwrap()
            ).collect());
        }

        Ok(raw_table)
    }
}
