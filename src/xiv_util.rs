use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Job {
    Gunbreaker,
    Paladin,
    Gladiator,
    DarkKnight,
    Warrior,
    Marauder,
    Scholar,
    Arcanist,
    Sage,
    Astrologian,
    WhiteMage,
    Conjurer,
    Samurai,
    Dragoon,
    Ninja,
    Monk,
    Reaper,
    Bard,
    Machinist,
    Dancer,
    BlackMage,
    BlueMage,
    Summoner,
    RedMage,
    Lancer,
    Pugilist,
    Rogue,
    Thaumaturge,
    Archer
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Role {
    Tank,
    DPS,
    Healer
}

#[derive(Debug)]
#[derive(Clone)]
pub struct PFListing {
    pub title: String,
    pub author: String,
    pub flags: String,
    pub description: String,
    pub slots: Vec<Slot>,
    pub last_updated: String,
    pub expires_in: String,
    pub min_ilvl: String,
    pub data_center: String,
    pub pf_category: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Slot {
    pub available_jobs: Vec<Job>,
    pub filled: bool,
}

#[allow(dead_code)]
impl Slot {   
    pub fn to_string(&self) -> String {
        // We don't want to disclose the secret
        format!("Slot({:#?}, {})", &self.available_jobs, &self.filled)
    }
    pub fn get_emoji_string(&self) -> String {
        if self.filled {
            match self.available_jobs.first() {
                Some(x) => x.get_emoji_string(),
                None => "".to_string()
            }
        } else {
            let contains_tank = self.available_jobs.iter().any(|x| x.get_role() == Role::Tank);
            let contains_healer = self.available_jobs.iter().any(|x| x.get_role() == Role::Healer);
            let contains_dps = self.available_jobs.iter().any(|x| x.get_role() == Role::DPS);

            if contains_tank && contains_healer && contains_dps {
                "<:tankhealerdps:985322491398459482>".to_string()
            } else if contains_tank && contains_healer && !contains_dps {
                "<:tankhealer:985322490375049246>".to_string()
            } else if contains_tank && !contains_healer && contains_dps {
                "<:tankdps:985322489422958662>".to_string()
            } else if contains_tank && !contains_healer && !contains_dps {
                "<:tank:985322488332443668>".to_string()
            } else if !contains_tank && contains_healer && contains_dps {
                "<:healerdps:985322474923233390>".to_string()
            } else if !contains_tank && contains_healer && !contains_dps {
                "<:healer:985322474134704138>".to_string()
            } else if !contains_tank && !contains_healer && contains_dps {
                "<:dps:985322470326280213>".to_string()
            } else {
                "".to_string()
            }
        }
    }
}

impl Job {
    pub fn get_emoji_string(&self) -> String {
        match self {
            Job::Gunbreaker => "<:gunbreaker:1313310646376468500>".to_string(),
            Job::Paladin => "<:paladin:1313310727796162560>".to_string(),
            Job::Gladiator => "<:gladiator:1313310643616485397>".to_string(),
            Job::DarkKnight => "<:darkknight:985322469873303624>".to_string(),
            Job::Warrior => "<:warrior:1313310816367411200>".to_string(),
            Job::Marauder => "<:marauder:1313310659500441620>".to_string(),
            Job::Scholar => "<:scholar:1313310742816096327>".to_string(),
            Job::Arcanist => "<:arcanist:1313310563350089798>".to_string(),
            Job::Sage => "<:sage:1313310737824878602>".to_string(),
            Job::Astrologian => "<:astrologian:1313310567217365012>".to_string(),
            Job::WhiteMage => "<:whitemage:1313310818829340792>".to_string(),
            Job::Conjurer => "<:conjurer:1313310574615986186>".to_string(),
            Job::Samurai => "<:samurai:1313310740228079656>".to_string(),
            Job::Dragoon => "<:dragoon:1313310582862250085>".to_string(),
            Job::Ninja => "<:ninja~1:1313310663342293012>".to_string(),
            Job::Monk => "<:monk:1313310661387878410>".to_string(),
            Job::Reaper => "<:reaper:1313310731541938266>".to_string(),
            Job::Bard => "<:bard:1313310568794427443>".to_string(),
            Job::Machinist => "<:machinist:1313310657491239004>".to_string(),
            Job::Dancer => "<:dancer~1:1313310576616804352>".to_string(),
            Job::BlackMage => "<:blackmage:1313310570530865183>".to_string(),
            Job::BlueMage => "<:bluemage:1313310572078436353>".to_string(),
            Job::Summoner => "<:summoner:1313310803038044201>".to_string(),
            Job::RedMage => "<:redmage:1313310733110345802>".to_string(),
            Job::Lancer => "<:lancer:1313310655453073449>".to_string(),
            Job::Pugilist => "<:pugilist:1313310729721352272>".to_string(),
            Job::Rogue => "<:rogue:1313310735761408030>".to_string(),
            Job::Thaumaturge => "<:thaumaturge:1313310813733261363>".to_string(),
            Job::Archer  => "<:archer:1313310565317480458>".to_string()
        }
    }

    pub fn get_role(&self) -> Role {
        let tanks = vec![Job::Paladin, Job::Gunbreaker, Job::DarkKnight, Job::Warrior, Job::Marauder, Job::Gladiator];
        let healers = vec![Job::Conjurer, Job::WhiteMage, Job::Scholar, Job::Astrologian, Job::Sage];

        if tanks.contains(self) {
            Role::Tank
        } else if healers.contains(self) {
            Role::Healer
        } else {
            Role::DPS
        }
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Job::Gunbreaker => write!(f, "Gunbreaker"),
            Job::Paladin => write!(f, "Paladin"),
            Job::Gladiator => write!(f, "Gladiator"),
            Job::DarkKnight => write!(f, "DarkKnight"),
            Job::Warrior => write!(f, "Warrior"),
            Job::Marauder => write!(f, "Marauder"),
            Job::Scholar => write!(f, "Scholar"),
            Job::Arcanist => write!(f, "Arcanist"),
            Job::Sage => write!(f, "Sage"),
            Job::Astrologian => write!(f, "Astrologian"),
            Job::WhiteMage => write!(f, "WhiteMage"),
            Job::Conjurer => write!(f, "Conjurer"),
            Job::Samurai => write!(f, "Samurai"),
            Job::Dragoon => write!(f, "Dragoon"),
            Job::Ninja => write!(f, "Ninja"),
            Job::Monk => write!(f, "Monk"),
            Job::Reaper => write!(f, "Reaper"),
            Job::Bard => write!(f, "Bard"),
            Job::Machinist => write!(f, "Machinist"),
            Job::Dancer => write!(f, "Dancer"),
            Job::BlackMage => write!(f, "BlackMage"),
            Job::BlueMage => write!(f, "BlueMage"),
            Job::Summoner => write!(f, "Summoner"),
            Job::RedMage => write!(f, "RedMage"),
            Job::Lancer => write!(f, "Lancer"),
            Job::Pugilist => write!(f, "Pugilist"),
            Job::Rogue => write!(f, "Rogue"),
            Job::Thaumaturge => write!(f, "Thaumaturge"),
            Job::Archer => write!(f, "Archer"),
        }
    }
}

impl FromStr for Job {

    type Err = ();

    fn from_str(input: &str) -> Result<Job, Self::Err> {
        match input {
            "PLD"  => Ok(Job::Paladin),
            "WAR"  => Ok(Job::Warrior),
            "DRK"  => Ok(Job::DarkKnight),
            "GNB"  => Ok(Job::Gunbreaker),
            "GLD"  => Ok(Job::Gladiator),
            "MRD"  => Ok(Job::Marauder),
            "WHM"  => Ok(Job::WhiteMage),
            "SCH"  => Ok(Job::Scholar),
            "AST"  => Ok(Job::Astrologian),
            "SGE"  => Ok(Job::Sage),
            "CNJ"  => Ok(Job::Conjurer),
            "ARN"  => Ok(Job::Arcanist),
            "MNK"  => Ok(Job::Monk),
            "PGL"  => Ok(Job::Pugilist),
            "DRG"  => Ok(Job::Dragoon),
            "LNC"  => Ok(Job::Lancer),
            "NIN"  => Ok(Job::Ninja),
            "ROG"  => Ok(Job::Rogue),
            "SAM"  => Ok(Job::Samurai),
            "RPR"  => Ok(Job::Reaper),
            "BRD"  => Ok(Job::Bard),
            "ARC"  => Ok(Job::Archer),
            "MCH"  => Ok(Job::Machinist),
            "DNC"  => Ok(Job::Dancer),
            "BLM"  => Ok(Job::BlackMage),
            "SMN"  => Ok(Job::Summoner),
            "BLU"  => Ok(Job::BlueMage),
            "RDM"  => Ok(Job::RedMage),
            "RGE"  => Ok(Job::Rogue),
            "THM"  => Ok(Job::Thaumaturge),
            "ACN"  => Ok(Job::Arcanist),

            _      => Err(()),
        }
    }
}

pub fn get_color_from_duty(duty_name: &str) -> u32 {
    match duty_name {
        "The Unending Coil of Bahamut (Ultimate)" => 0xfce100,
        "The Weapon's Refrain (Ultimate)" => 0x008bfc,
        "The Epic of Alexander (Ultimate)" => 0xfcaa00,
        "Dragonsong's Reprise (Ultimate)" =>  0xf12916,
        _ =>  0xf0a057
    }
}
