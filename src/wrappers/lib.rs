#![crate_name = "wrappers"]

// #![forbid(unsafe_code)]
/* Is there a safe way to remember each party's instance without converting to a JS type? */
#[macro_use] extern crate lazy_static;

extern crate common;
extern crate crypto;
extern crate protocol;

use crypto::prelude::TPayload;
use protocol::private_id::{partner::PartnerPrivateId, traits::*};
use protocol::private_id::{company::CompanyPrivateId, traits::CompanyPrivateIdProtocol};

use std::sync::Mutex;

macro_rules! MUT { ($var:expr) => { $var.lock().unwrap() } }

lazy_static! {
    static ref PARTNER: Mutex<PartnerPrivateId> = Mutex::new(Default::default());
    static ref COMPANY: Mutex<CompanyPrivateId> = Mutex::new(Default::default());
}

//absolute beginner fix
pub fn partner_step_2(partner_input: &str) {
    if MUT!(PARTNER).load_json(partner_input, false).is_err() {
        panic!("Attempted to run the protocol after the text was already initialized.");
        /* default the PartnerPrivateId instance here if we want to allow reruns */
    }
}

pub fn partner_step_3() {
    MUT!(PARTNER).gen_permute_pattern().unwrap();
}

pub fn partner_step_4() -> TPayload {
    MUT!(PARTNER).permute_hash_to_bytes().unwrap()
}

pub fn company_step_5(company_input: &str) {
    if MUT!(COMPANY).load_json(company_input, false) == false {
       panic!("Attempted to run the protocol after the text was already initialized.");
       /* default the CompanyPrivateId instance here if we want to allow reruns */
    }
    // MUT!(COMPANY).gen_permute_pattern().unwrap();
}

pub fn company_step_6() -> TPayload {
    MUT!(COMPANY).get_permuted_keys().unwrap()
}

pub fn partner_step_7(u_company: TPayload) -> (TPayload, TPayload) {
    MUT!(PARTNER).encrypt_permute(u_company)
}

pub fn company_step_8(u_partner: TPayload) {
    MUT!(COMPANY).set_encrypted_partner_keys(u_partner).unwrap();
}

pub fn company_step_9a(e_company: TPayload) {
    MUT!(COMPANY).set_encrypted_company("e_company".to_string(), e_company).unwrap();
}

pub fn company_step_9b(v_company: TPayload) {
    MUT!(COMPANY).set_encrypted_company("v_company".to_string(), v_company).unwrap();
}

pub fn company_step_10() -> TPayload {
    MUT!(COMPANY).get_encrypted_partner_keys().unwrap()
}

pub fn company_step_11() {
    MUT!(COMPANY).calculate_set_diff().unwrap();
}

pub fn company_step_12() -> TPayload {
    MUT!(COMPANY).get_set_diff_output("s_prime_partner".to_string()).unwrap()
}

pub fn company_step_13() -> TPayload {
    MUT!(COMPANY).get_set_diff_output("s_prime_company".to_string()).unwrap()
}

pub fn company_step_14(s_prime_partner: TPayload, not_matched_val: Option<&str>) {
    MUT!(COMPANY).write_partner_to_id_map(s_prime_partner, not_matched_val.map(String::from).as_ref()).unwrap();
}

pub fn company_step_15() {
    MUT!(COMPANY).write_company_to_id_map();
}

pub fn company_print_output(use_row_numbers: bool) {
    MUT!(COMPANY).print_id_map(u32::MAX as usize, false, use_row_numbers);
}

pub fn company_build_output(use_row_numbers: bool) -> String {
    MUT!(COMPANY).stringify_id_map(use_row_numbers)
}

pub fn partner_step_14(s_prime_partner: TPayload) -> TPayload {
    MUT!(PARTNER).encrypt(s_prime_partner).unwrap()
}

pub fn partner_step_15(v_partner: TPayload, s_prime_company: TPayload, not_matched_val: Option<&str>) {
    MUT!(PARTNER).create_id_map(v_partner, s_prime_company, not_matched_val);
}

pub fn partner_print_output(use_row_numbers: bool) {
    MUT!(PARTNER).print_id_map(usize::MAX, false, use_row_numbers);
}

pub fn partner_build_output(use_row_numbers: bool) -> String  {
    MUT!(PARTNER).stringify_id_map(use_row_numbers)
}

// pub fn test() {
//     __test();
// }