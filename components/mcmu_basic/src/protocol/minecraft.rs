use rand::Rng;
//use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write};

pub const UNCONNECTED_PING: &[u8] = &[
    1, 0, 0, 0, 0, 0, 0, 11, 231, 0, 255, 255, 0, 254, 254, 254, 254, 253, 253, 253, 253, 18, 52,
    86, 120, 144, 237, 101, 30, 245, 192, 252, 166,
];
pub const UNCONNECTED_PONG_PREFIX: &[u8] = &[
    28, 0, 0, 0, 0, 0, 209, 254, 33, 128, 245, 64, 50, 135, 169, 155, 178, 0, 255, 255, 0, 254,
    254, 254, 254, 253, 253, 253, 253, 18, 52, 86, 120, 0, 93,
];

#[derive(Clone, Debug)]
pub struct UnconnectedPong<'a> {
    pub prefix: Cow<'a, [u8]>,
    pub _mcpe_symbol: (),
    pub player_name: Cow<'a, str>,
    pub _unknown_field1: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub is_client: bool,
    pub max_players: u32,
    pub session_id: u64,
    pub world_name: Cow<'a, str>,
    pub game_mode: Cow<'a, str>,
    pub _unknown_field2: Cow<'a, str>,
    pub ipv4_port: u16,
    pub ipv6_port: u16,
}

impl<'a> UnconnectedPong<'a> {
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < UNCONNECTED_PONG_PREFIX.len() {
            return None;
        }

        let (prefix, data) = data.split_at(UNCONNECTED_PONG_PREFIX.len());
        let prefix = Cow::Borrowed(prefix);
        let mut iter = std::str::from_utf8(&data).ok()?.split(";");
        let _mcpe_symbol = match iter.next()? {
            "MCPE" => (),
            _ => return None,
        };
        let player_name = Cow::Borrowed(iter.next()?);
        let _unknown_field1 = Cow::Borrowed(iter.next()?);
        let version = Cow::Borrowed(iter.next()?);
        let is_client = iter.next()? == "1";
        let max_players = iter.next()?.parse().ok()?;
        let session_id = iter.next()?.parse().ok()?;
        let world_name = Cow::Borrowed(iter.next()?);
        let game_mode = Cow::Borrowed(iter.next()?);
        let _unknown_field2 = Cow::Borrowed(iter.next()?);
        let ipv4_port = iter.next()?.parse().ok()?;
        let ipv6_port = iter.next()?.parse().ok()?;

        Some(UnconnectedPong {
            prefix,
            _mcpe_symbol,
            player_name,
            _unknown_field1,
            version,
            is_client,
            max_players,
            session_id,
            world_name,
            game_mode,
            _unknown_field2,
            ipv4_port,
            ipv6_port,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let UnconnectedPong {
            prefix,
            _mcpe_symbol,
            player_name,
            _unknown_field1,
            version,
            is_client,
            max_players,
            session_id,
            world_name,
            game_mode,
            _unknown_field2,
            ipv4_port,
            ipv6_port,
        } = self;

        let _mcpe_symbol = "MCPE";
        let is_client = if *is_client { "1" } else { "0" };
        let max_players = max_players.to_string();
        let session_id = session_id.to_string();
        let ipv4_port = ipv4_port.to_string();
        let ipv6_port = ipv6_port.to_string();

        macro_rules! _foo{
            ($($id:ident),*)=>{
                {
                    let mut vec=Vec::<u8>::with_capacity($($id.len()+1+)*0);
                    vec.write_all(prefix).unwrap();
                    $(
                        vec.write_all($id.as_bytes()).unwrap();
                        vec.write_all(b";").unwrap();
                    )*
                    vec
                }
            }
        }

        _foo!(
            _mcpe_symbol,
            player_name,
            _unknown_field1,
            version,
            is_client,
            max_players,
            session_id,
            world_name,
            game_mode,
            _unknown_field2,
            ipv4_port,
            ipv6_port
        )
    }

    pub fn clean(&mut self) {
        self.session_id = 0;
        self.prefix = Cow::Borrowed(UNCONNECTED_PONG_PREFIX);
        self.ipv4_port = 0;
        self.ipv6_port = 0;
    }

    pub fn random_session_id(&mut self) {
        const BASE: u64 = 10000_00000_00000_00000;
        self.session_id = rand::thread_rng().gen_range(BASE..u64::MAX);
    }
}

#[test]
pub fn test() {
    const DATA: &[u8] = &[
        28, 0, 0, 0, 0, 0, 0, 11, 231, 172, 241, 198, 123, 248, 129, 183, 49, 0, 255, 255, 0, 254,
        254, 254, 254, 253, 253, 253, 253, 18, 52, 86, 120, 0, 83, 77, 67, 80, 69, 59, 70, 97, 110,
        99, 121, 70, 108, 97, 109, 101, 59, 52, 52, 56, 59, 49, 46, 49, 55, 46, 49, 49, 59, 49, 59,
        53, 59, 49, 49, 57, 49, 56, 50, 53, 53, 51, 54, 52, 54, 49, 50, 50, 48, 51, 57, 53, 49, 59,
        231, 140, 170, 229, 164, 180, 59, 67, 114, 101, 97, 116, 105, 118, 101, 59, 49, 59, 53, 56,
        50, 48, 55, 59, 51, 52, 52, 48, 48, 59,
    ];
    let pong = UnconnectedPong::parse(DATA).unwrap();
    //println!("{:#?}",pong);
    assert_eq!(pong.to_bytes(), DATA);
    println!("{:#?}", UnconnectedPong::parse(&pong.to_bytes()));
}
