use crate::cpu::Register;
use crate::util::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Branch {
    BranchAndExchange(Register),
    Branch { offset: i32, link: bool },
}

impl Branch {
    pub fn from_bits_bx(bits: u32) -> Branch {
        Branch::BranchAndExchange(bits.get_register(Offset(0)))
    }

    pub fn from_bits_branch(bits: u32) -> Branch {
        let raw_bits: u32 = bits.get_bits(Offset(0), Length(24));
        let raised: i32 = (raw_bits << 8) as i32;
        let offset = raised >> 8;

        Branch::Branch {
            offset,
            link: bits.is_flag_set(Offset(24)),
        }
    }
}

#[test]
fn branch_and_exchange() {
    let decoded = Branch::from_bits_bx(0b1101);
    assert_eq!(
        decoded,
        Branch::BranchAndExchange(Register::from_index(0b1101))
    )
}

#[test]
fn branch() {
    let decoded = Branch::from_bits_branch(0b1_010100100010110101010010);
    assert_eq!(
        decoded,
        Branch::Branch {
            offset: 0b010100100010110101010010,
            link: true,
        }
    );

    let decoded = Branch::from_bits_branch(0b0_111111111111111111111111);
    assert_eq!(
        decoded,
        Branch::Branch {
            offset: -1,
            link: false,
        }
    )
}
