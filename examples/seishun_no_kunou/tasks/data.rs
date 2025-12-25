use super::oumae_kumiko::Run;
use super::uji_bashi::Cry;

#[derive(Clone)]
pub enum Data {
    OumaeKumiko(Run),
    UjiBashi(Cry),
}

impl Data {
    pub fn oumae_kumiko(self) -> Result<Run, ()> {
        let Self::OumaeKumiko(run) = self else {
            Err(())?
        };
        Ok(run)
    }

    pub fn uji_bashi(self) -> Result<Cry, ()> {
        let Self::UjiBashi(cry) = self else { Err(())? };
        Ok(cry)
    }
}
