extern crate rustc_serialize;

struct Args {
    host: String,
    port: u16,
    token: String,
}

impl Args {
    fn parse() -> Self {
        let mut args = std::env::args();
        args.next().unwrap();
        let host = args.next().unwrap_or("127.0.0.1".to_owned());
        let port = args
            .next()
            .map_or(31001, |s| s.parse().expect("Can't parse port"));
        let token = args.next().unwrap_or("0000000000000000".to_string());
        Self { host, port, token }
    }
}

fn main() -> std::io::Result<()> {
    use trans::Trans;
    use std::io::Write;

    let args = Args::parse();
    let stream = std::net::TcpStream::connect((args.host.as_str(), args.port))?;

    stream.set_nodelay(true)?;

    let stream_clone = stream.try_clone()?;
    let mut reader = std::io::BufReader::new(stream);
    let mut writer = std::io::BufWriter::new(stream_clone);

    args.token.write_to(&mut writer)?;
    writer.flush()?;

    let mut first = true;

    loop {
        let message = model::ServerMessageGame::read_from(&mut reader)?;
        let mut player_view = match message.player_view {
            Some(view) => view,
            None => break,
        };
        if first {
            first = false;
        } else {
            player_view.game.level.tiles.clear();
        }
        let json = rustc_serialize::json::encode(&player_view).expect("Can't serialize player_view");
        write!(&mut std::io::stdout(), "{}\n", json)?;
        let message = model::PlayerMessageGame::ActionMessage {
            action: model::Versioned { inner: std::collections::HashMap::new() },
        };
        message.write_to(&mut writer)?;
        writer.flush()?;
    }

    Ok(())
}
