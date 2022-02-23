use anyhow::Result;
use json::JsonValue;
use smallvec::SmallVec;
use std::{fs, path::PathBuf, str::FromStr};

lazy_static! {
    //储存文件夹
    pub static ref STORAGE_PATH: PathBuf = {
        let mut p = match home::home_dir() {
            Some(n) => n,
            None => std::env::current_dir().unwrap_or(PathBuf::new()),
        };
        p.push("mcmu_storage");
        p
    };

    //配置文件
    static ref PROFILE_PATH: PathBuf = {
        let mut p = STORAGE_PATH.clone();
        p.push("mcmu_profile.json");
        p
    };

    //加载时可能产生错误
    static ref _PROFILE: std::result::Result<JsonValue,String> = {
        match fs::read_to_string(&*PROFILE_PATH){
            Ok(file)=>json::parse(&file).map_err(|err|format!("Cannot parse profile file: {}", err)),
            Err(err)=>Err(format!("Cannot get profile: {}",err))
        }
    };

    static ref PROFILE:&'static JsonValue=_PROFILE.as_ref().expect("Check PROFILE_ERR before using PROFILE!!!");

    static ref PROFILE_ERR:Option<&'static String>=match &*_PROFILE{
        Ok(_)=>None,
        Err(err)=>Some(err)
    };
}

#[inline]
fn parse_path(path: &str) -> Result<SmallVec<[&str; 10]>> {
    Ok(path.split(".").collect())
}

pub fn set_value(path: &str, value: String, try_create_path: bool) -> Result<()> {
    let mut pf = PROFILE.clone(); //mutable
    let mut prv = &mut pf;

    let names = parse_path(path)?;
    for (index, x) in names.iter().enumerate() {
        match prv {
            JsonValue::Object(obj) => {
                //如果是最后一个元素则赋值
                if index == names.len() - 1 {
                    obj.insert(x, JsonValue::String(value));
                    break;
                }

                if obj.get_mut(x).is_some() {
                    prv = obj.get_mut(x).unwrap();
                } else {
                    //该object下没有该键，是否尝试创建路径
                    if try_create_path {
                        obj.insert(x, JsonValue::new_object());
                        prv = obj.get_mut(x).unwrap();
                    } else {
                        return Err(anyhow!("Path `{}` not exist", names[..index + 1].join(".")));
                    }
                }
            }
            _ => {
                return Err(anyhow!(
                    "Path `{}` is not an object",
                    names[..index + 1].join(".")
                ))
            }
        }
    }
    let data = json::stringify_pretty(pf, 4);
    Ok(fs::write(&*PROFILE_PATH, data.as_bytes())?)
}

pub fn get_and_parse<T: FromStr>(path: &str) -> Result<Option<T>> {
    let names = parse_path(path)?;
    let mut prv = &**PROFILE;

    for (index, x) in names.iter().enumerate() {
        match prv {
            JsonValue::Object(obj) => match obj.get(x) {
                Some(jv) => prv = jv,
                None => return Ok(None),
            },
            _ => {
                return Err(anyhow!(
                    "Path `{}` is not an object",
                    names[..index + 1].join(".")
                ))
            }
        }
    }

    match prv.as_str() {
        Some(s) => match s.parse() {
            Ok(t) => Ok(Some(t)),
            Err(_) => Err(anyhow!("Value at `{}` is invalid", path)),
        },
        None => Err(anyhow!("Value at `{}` is expected to be a string", path)),
    }
}
