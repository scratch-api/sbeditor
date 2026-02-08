use itertools::Itertools;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct BroadcastPrim {
    pub name: String,
    pub id: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
#[derive(Debug, Clone)]
pub enum Prim {
    Number(f64),
    PositiveNumber(f64),
    PositiveInteger(u64),
    Integer(i64),
    Angle(f64),
    Color(String),
    String(String),
    Broadcast(BroadcastPrim),
    Other(u64),
}
impl Prim {
    pub fn opcode(self) -> Option<&'static str> {
        // i literally don't know of a better way of doing this...
        match self {
            Self::Number(_) => Some("math_number"),
            Self::PositiveNumber(_) => Some("math_positive_number"),
            Self::PositiveInteger(_) => Some("math_whole_number"),
            Self::Integer(_) => Some("math_integer"),
            Self::Angle(_) => Some("math_angle"),
            Self::Color(_) => Some("colour_picker"),
            Self::String(_) => Some("text"),
            Self::Broadcast(_) => Some("event_broadcast_menu"),
            _ => None, // data_variable
                       // data_listcontents
        }
    }
}
pub fn deserialize_prim<'de, D>(deserializer: D) -> Result<Prim, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    if raw.len() < 2 {
        return Err(serde::de::Error::custom(
            "Prim lacking elems, expected 2+ elems",
        ));
    }
    let id = raw[0].as_u64().ok_or_else(|| {
        serde::de::Error::custom(format!("Expected prim id to be u64, got {}", raw[0]))
    })?;
    match id {
        // TODO: test with a lot of projects
        // because of js's weak and dynamic typing, you can't be sure what the types are here.
        // just cast them to the correct type and try to avoid erroring.
        4 => {
            let val = raw[1].as_f64().unwrap_or(0f64);
            Ok(Prim::Number(val))
        }
        5 => {
            let val = raw[1].as_f64().unwrap_or(0f64);
            // file an issue on gh if you want to re-enable to following check
            // if val < 0. {
            //     eprintln!("Expected positive number prim val, got {}", val)
            // }
            Ok(Prim::PositiveNumber(val))
        }
        6 => {
            let val = raw[1].as_u64().unwrap_or(0u64);
            Ok(Prim::PositiveInteger(val))
        }
        7 => {
            let val = raw[1].as_i64().unwrap_or(0i64);
            Ok(Prim::Integer(val))
        }
        8 => {
            let val = raw[1].as_f64().unwrap_or(0f64);
            Ok(Prim::Angle(val))
        }
        9 => {
            let val = raw[1].as_str().unwrap_or("#000000");
            Ok(Prim::Color(val.to_string()))
        }
        10 => {
            let val = raw[1].as_str().unwrap_or("");
            Ok(Prim::String(val.to_string()))
        }
        11 => {
            if raw.len() != 3 && raw.len() != 5 {
                return Err(serde::de::Error::custom(format!(
                    "Expected 3 or 5 elems, got {}",
                    raw.len()
                )));
            }
            let name = raw[1].as_str().ok_or_else(|| {
                serde::de::Error::custom(format!("Expected name (str), got {}", raw[1]))
            })?;
            let id = raw[2].as_str().ok_or_else(|| {
                serde::de::Error::custom(format!("Expected id (str), got {}", raw[2]))
            })?;
            let x = if let Some(x) = raw.get(3) {
                Some(x.as_f64().ok_or_else(|| {
                    serde::de::Error::custom(format!("Expected x (f64), got {}", x))
                })?)
            } else {
                None
            };
            let y = if let Some(x) = raw.get(4) {
                Some(x.as_f64().ok_or_else(|| {
                    serde::de::Error::custom(format!("Expected y (f64), got {}", x))
                })?)
            } else {
                None
            };
            Ok(Prim::Broadcast(BroadcastPrim {
                name: name.to_string(),
                id: id.to_string(),
                x,
                y,
            }))
        }
        _ => {
            eprintln!("Unknown prim#{}, data=[{}]", id, raw.iter().join(", "));
            Ok(Prim::Other(id))
        }
    }
}
