use crate::data_types::basic_stats;

// The stat sheet for a standard human person.
pub struct HumanSheet {
    pub bio: basic_stats::BasicBioInfo,
    pub abilities: basic_stats::BasicAbilities,
    pub skills: basic_stats::BasicSkills,
}
