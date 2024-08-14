pub mod fairyring {
    pub mod common {
        include!("prost/fairyring.common.rs");
    }

    pub mod keyshare {
        include!("prost/fairyring.keyshare.rs");
    }

    pub mod pep {
        include!("prost/fairyring.pep.rs");
    }
}
