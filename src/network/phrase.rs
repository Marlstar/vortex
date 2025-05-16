use std::net::{IpAddr, Ipv4Addr};
use crate::Error;

pub fn local_ip() -> Ipv4Addr {
    return match local_ip_address::local_ip().unwrap() {
        IpAddr::V4(ip) => ip,
        IpAddr::V6(_) => panic!("local_ip (an ipv4 function) somehow returned an ipv6"),
    }

}

pub static WORDLIST: [&str; 256] = [
    "abyss","alchemy","amethyst","android","angel","apex","arcane","armor","asteroid","aurora","avalanche","axiom","banshee","barbarian","basilisk","beacon","berserker","blackout","blade","blizzard","blossom","bolt","bot","bramble","brew","bronze","bullet","cactus","cannon","celestial","chaos","charm","chasm","chrome","cinder","cliff","cloak","comet","cosmic","crimson","crypt","crystal","cyber","dagger","dancer","darkness","dawn","delta","demon","descent","dragon","dream","druid","dune","dust","dynamo","eagle","echo","ember","emerald","enchant","engine","epic","epoch","equinox","fable","falcon","fang","feather","flare","flint","forge","frost","fury","galaxy","ghost","glimmer","glitch","gnome","goblin","golem","gravity","grotto","guardian","halo","hammer","havoc","hawk","hex","hollow","horizon","howl","hunter","hydra","iceberg","ignite","illusion","inferno","ion","jade","jester","jewel","jinx","jungle","karma","keeper","kelp","knight","kraken","lancer","lava","legend","levitate","lightning","lore","lucid","lunar","lynx","mage","magnet","maiden","mantra","marble","maze","meadow","mechanic","mermaid","meteor","mirage","moonlight","morph","mosh","mystic","nebula","nexus","nimbus","ninja","nomad","nova","nyx","obsidian","omega","onix","onyx","oracle","orbit","origin","paladin","panther","paradox","pegasus","phantom","phoenix","pine","plasma","portal","potion","prism","prophet","psi","pyre","python","quantum","quartz","quest","quill","rage","raptor","raven","reactor","remnant","riddle","rift","rogue","rose","rune","sage","sandstorm","sapphire","scarlet","scorch","scroll","shade","shadow","shard","shepherd","shimmer","signal","silver","skyline","smoke","snare","snowfall","solstice","sorcerer","spectrum","speed","spell","spider","spike","spirit","spire","sprite","starlight","steel","storm","strike","surge","talisman","tarot","tempest","temple","thief","thorn","thunder","tiger","titan","tome","torch","tornado","totem","treasure","tremor","trident","tundra","turbo","unicorn","unity","valor","vanish","vault","veil","venom","viper","void","vortex","wanderer","warp","warrior","wave","whirlwind","whisper","wildfire","willow","winter","witch","wizard","wolf","wrath","xenon","yeti","zealot","zenith","zephyr","zodiac","zombie"
];

pub fn word_from_num(num: u8) -> &'static str {
    return WORDLIST[num as usize]
}
pub fn num_from_word(word: &str) -> u8 {
    return WORDLIST.iter().position(|&a| a == word).unwrap() as u8
}

pub fn ipv4_phrase(ip: impl Into<Ipv4Addr>) -> String {
    let ip = std::convert::Into::<Ipv4Addr>::into(ip);
    let words = ip.octets().iter().map(|&a| word_from_num(a)).collect::<Vec<&'static str>>();
    return format!("{}-{}-{}-{}", words[0], words[1], words[2], words[3]);
}

pub fn phrase_to_ipv4(phrase: &str) -> Ipv4Addr {
    let octets = phrase.split('-').map(num_from_word).collect::<Vec<u8>>();
    return [octets[0], octets[1], octets[2], octets[3]].into()
}

pub fn arg_to_ipv4(arg: &str) -> Result<Ipv4Addr, std::io::Error> {
    if arg.starts_with(['0','1','2','3','4','5','6','7','8','9']) {
        let octets = arg.split('.').map(|a| a.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        return Ok([octets[0], octets[1], octets[2], octets[3]].into())
    } else {
        Ok(phrase_to_ipv4(arg))
    }
}
