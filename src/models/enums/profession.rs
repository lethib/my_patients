use crate::models::_entities::sea_orm_active_enums::Profession;

impl Profession {
  pub fn to_french(&self) -> &str {
    match self {
      Self::GeneralPractitioner => "MÉDECIN GÉNÉRALISTE",
      Self::Pediatrician => "PÉDIATRE",
      Self::Gynecologist => "GYNÉCOLOGUE",
      Self::Psychiatrist => "PSYCHIATRE",
      Self::Gastroenterologist => "GASTRO-ENTÉROLOGUE",
      Self::EntSpecialist => "ORL",
      Self::Endocrinologist => "ENDOCRINOLOGUE",
      Self::Cardiologist => "CARDIOLOGUE",
      Self::Angiologist => "ANGIOLOGUE",
      Self::Nephrologist => "NÉPHROLOGUE",
      Self::Neurologist => "NEUROLOGUE",
      Self::Pulmonologist => "PNEUMOLOGUE",
      Self::Rheumatologist => "RHUMATOLOGUE",
      Self::Dermatologist => "DERMATOLOGUE",
      Self::Dentist => "DENTISTE",
      Self::Midwife => "SAGE-FEMME",
      Self::Physiotherapist => "KINÉSITHÉRAPEUTE",
      Self::Nurse => "INFIRMIER/INFIRMIÈRE",
      Self::Psychologist => "PSYCHOLOGUE",
      Self::Osteopath => "OSTÉOPATHE D.O.",
      Self::Audiologist => "AUDIOPROTHÉSISTE",
      Self::Chiropractor => "CHIROPRACTEUR",
      Self::GeneticCounselor => "CONSEILLER GÉNÉTIQUE",
      Self::Dietitian => "DIÉTÉTICIEN",
      Self::OccupationalTherapist => "ERGOTHÉRAPEUTE",
      Self::SpeechTherapist => "ORTHOPHONISTE",
      Self::Orthoptist => "ORTHOPTISTE",
      Self::Podiatrist => "PODOLOGUE",
      Self::Psychomotrician => "PSYCHOMOTRICIEN",
      Self::Psychotherapist => "PSYCHOTHÉRAPEUTE",
    }
  }
}
