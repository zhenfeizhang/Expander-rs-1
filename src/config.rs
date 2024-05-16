#[derive(Debug, Clone, PartialEq)]
pub enum PolynomialCommitmentType {
    Raw,
    KZG,
    Orion,
    FRI,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    M31,
    BabyBear,
    BN254,
}

const M31_PACK_SIZE: usize = 8; // TODO: update to load from static function of M31

#[derive(Debug, Clone)]
pub enum FiatShamirHashType {
    SHA256,
    Keccak256,
    Poseidon,
    Animoe,
    MIMC7,
}

#[derive(Debug, Clone)]
pub struct Config {
    num_repetitions: usize,
    vectorize_size: usize,

    field_size: usize,
    security_bits: usize,
    grinding_bits: usize,
    num_parallel: usize, // nb_parallel

    polynomial_commitment_type: PolynomialCommitmentType,
    field_type: FieldType, // LATER: consider infer this from trait
    fs_hash: FiatShamirHashType,
}

impl Config {
    pub fn new() -> Self {
        let mut vectorize_size = 0;

        let security_bits = 100;
        let grinding_bits = 10;
        let num_parallel = 16;

        let field_size = match FieldType::M31 {
            FieldType::M31 => {
                vectorize_size = num_parallel / M31_PACK_SIZE;
                31
            }
            FieldType::BabyBear => 31,
            FieldType::BN254 => 254,
        };

        let num_repetitions = (security_bits - grinding_bits + field_size - 1) / field_size;

        let polynomial_commitment_type = PolynomialCommitmentType::Raw;
        let field_type = FieldType::M31;
        let fs_hash = FiatShamirHashType::SHA256;

        if polynomial_commitment_type == PolynomialCommitmentType::KZG {
            assert_eq!(field_type, FieldType::BN254);
        }

        Config {
            num_repetitions, // update later
            vectorize_size,  // update later
            field_size,      // update later
            security_bits,
            grinding_bits,
            num_parallel,
            polynomial_commitment_type,
            field_type,
            fs_hash,
        }
    }
    pub fn get_num_repetitions(&self) -> usize {
        self.num_repetitions
    }
}
