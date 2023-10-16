#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlaneRectangularCoordinateSystem {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    SIXTH,
    SEVENTH,
    EIGHTH,
    NINTH,
    TENTH,
    ELEVENTH,
    TWELFTH,
    THIRTEENTH,
    FOURTEENTH,
    FIFTEENTH,
    SIXTEENTH,
    SEVENTEENTH,
    EIGHTEENTH,
    NINETEENTH,
}

#[derive(Debug)]
pub struct Origin {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct EpsgInfo {
    pub jgd2000: String,
    pub jgd2011: String,
}

impl PlaneRectangularCoordinateSystem {
    pub fn origin(&self) -> Origin {
        match self {
            Self::FIRST => Origin { x: 129.5, y: 33.0 },
            Self::SECOND => Origin { x: 131.0, y: 33.0 },
            Self::THIRD => Origin {
                x: 132.166667,
                y: 36.0,
            },
            Self::FOURTH => Origin { x: 133.5, y: 33.0 },
            Self::FIFTH => Origin {
                x: 134.333333,
                y: 36.0,
            },
            Self::SIXTH => Origin { x: 136.0, y: 36.0 },
            Self::SEVENTH => Origin {
                x: 137.166667,
                y: 36.0,
            },
            Self::EIGHTH => Origin { x: 138.5, y: 36.0 },
            Self::NINTH => Origin { x: 139.5, y: 36.0 },
            Self::TENTH => Origin { x: 140.5, y: 40.0 },
            Self::ELEVENTH => Origin { x: 140.25, y: 44.0 },
            Self::TWELFTH => Origin { x: 142.25, y: 44.0 },
            Self::THIRTEENTH => Origin { x: 144.25, y: 44.0 },
            Self::FOURTEENTH => Origin { x: 142.0, y: 26.0 },
            Self::FIFTEENTH => Origin { x: 127.5, y: 26.0 },
            Self::SIXTEENTH => Origin { x: 124.0, y: 26.0 },
            Self::SEVENTEENTH => Origin { x: 131.0, y: 26.0 },
            Self::EIGHTEENTH => Origin { x: 136.0, y: 20.0 },
            Self::NINETEENTH => Origin { x: 154.0, y: 26.0 },
        }
    }

    pub fn epsg(&self) -> EpsgInfo {
        match self {
            Self::FIRST => EpsgInfo {
                jgd2000: "EPSG:2443".to_string(),
                jgd2011: "EPSG:6669".to_string(),
            },
            Self::SECOND => EpsgInfo {
                jgd2000: "EPSG:2444".to_string(),
                jgd2011: "EPSG:6670".to_string(),
            },
            Self::THIRD => EpsgInfo {
                jgd2000: "EPSG:2445".to_string(),
                jgd2011: "EPSG:6671".to_string(),
            },
            Self::FOURTH => EpsgInfo {
                jgd2000: "EPSG:2446".to_string(),
                jgd2011: "EPSG:6672".to_string(),
            },
            Self::FIFTH => EpsgInfo {
                jgd2000: "EPSG:2447".to_string(),
                jgd2011: "EPSG:6673".to_string(),
            },
            Self::SIXTH => EpsgInfo {
                jgd2000: "EPSG:2448".to_string(),
                jgd2011: "EPSG:6674".to_string(),
            },
            Self::SEVENTH => EpsgInfo {
                jgd2000: "EPSG:2449".to_string(),
                jgd2011: "EPSG:6675".to_string(),
            },
            Self::EIGHTH => EpsgInfo {
                jgd2000: "EPSG:2450".to_string(),
                jgd2011: "EPSG:6676".to_string(),
            },
            Self::NINTH => EpsgInfo {
                jgd2000: "EPSG:2451".to_string(),
                jgd2011: "EPSG:6677".to_string(),
            },
            Self::TENTH => EpsgInfo {
                jgd2000: "EPSG:2452".to_string(),
                jgd2011: "EPSG:6678".to_string(),
            },
            Self::ELEVENTH => EpsgInfo {
                jgd2000: "EPSG:2453".to_string(),
                jgd2011: "EPSG:6679".to_string(),
            },
            Self::TWELFTH => EpsgInfo {
                jgd2000: "EPSG:2454".to_string(),
                jgd2011: "EPSG:6680".to_string(),
            },
            Self::THIRTEENTH => EpsgInfo {
                jgd2000: "EPSG:2455".to_string(),
                jgd2011: "EPSG:6681".to_string(),
            },
            Self::FOURTEENTH => EpsgInfo {
                jgd2000: "EPSG:2456".to_string(),
                jgd2011: "EPSG:6682".to_string(),
            },
            Self::FIFTEENTH => EpsgInfo {
                jgd2000: "EPSG:2457".to_string(),
                jgd2011: "EPSG:6683".to_string(),
            },
            Self::SIXTEENTH => EpsgInfo {
                jgd2000: "EPSG:2458".to_string(),
                jgd2011: "EPSG:6684".to_string(),
            },
            Self::SEVENTEENTH => EpsgInfo {
                jgd2000: "EPSG:2459".to_string(),
                jgd2011: "EPSG:6685".to_string(),
            },
            Self::EIGHTEENTH => EpsgInfo {
                jgd2000: "EPSG:2460".to_string(),
                jgd2011: "EPSG:6686".to_string(),
            },
            Self::NINETEENTH => EpsgInfo {
                jgd2000: "EPSG:2461".to_string(),
                jgd2011: "EPSG:6687".to_string(),
            },
        }
    }
}
