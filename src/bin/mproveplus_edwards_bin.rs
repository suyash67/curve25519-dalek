#![allow(non_snake_case)]

extern crate structopt;
extern crate curve25519_dalek;
extern crate time;

use structopt::StructOpt;
use time::{PreciseTime};
use curve25519_dalek::mproveplus::proofs::mprove_plus_edwards::MProvePlus;

#[derive(Debug, StructOpt)]
#[structopt(name = "mprove-plus", about = "MProvePlus proof generation simulator using Edwards curve.")]
struct Opt {
  //#[structopt(short = "a", long = "anonsize")]
  anon_list_size: usize,
  //#[structopt(short = "o", long = "ownsize")]
  own_list_size: usize,
  #[structopt(short = "n", long = "numiter", default_value = "1")]
  num_iter: u32,
}

fn main() {
    // 
    // cargo run --release --bin mproveplus_edwards_bin 1000 100 -n 10
    //
    let opt = Opt::from_args();
    let n = opt.anon_list_size;
    let s = opt.own_list_size;
    let num_iter = opt.num_iter;
    let N = (((n*s + 2*n + s + 3) as u64).next_power_of_two()) as f64;
    println!("Options = {:?}", opt);

    let mut gen_proof_start;
    let mut gen_proof_end;
    let mut ver_proof_start;
    let mut ver_proof_end;
    let mut total_gen_proof_duration: i64 = 0;
    let mut total_ver_proof_duration: i64 = 0;
    let mut mprove_plus_proof;
    let mut result;

    // Run a sample to estimate timing fir given n and s
    let est_n = 100;
    let est_s = 10;
    let est_N = (((est_n*est_s + 2*est_n + est_s + 3) as u64).next_power_of_two()) as f64;
    let (est_G, est_H, est_Gt, est_H_prime, est_p_vec, est_g_prime_vec, est_h_vec, est_g_vec_append, est_h_vec_append, est_C_vec_mut, est_P_vec, est_H_vec, est_E_vec, est_x_vec, est_gamma) = MProvePlus::gen_params(est_n, est_s);
    for _i in 0..5 {        

      gen_proof_start = PreciseTime::now();
      mprove_plus_proof = MProvePlus::prove(&est_G, &est_H, &est_Gt, &est_H_prime, &est_p_vec, &est_g_prime_vec, &est_h_vec, &est_g_vec_append, &est_h_vec_append, &est_C_vec_mut, &est_P_vec, &est_H_vec, &est_E_vec, &est_x_vec, &est_gamma);
      gen_proof_end = PreciseTime::now();
      total_gen_proof_duration += (gen_proof_start.to(gen_proof_end)).num_milliseconds();

      ver_proof_start = PreciseTime::now();
      result = mprove_plus_proof.verify(&est_G, &est_H, &est_Gt, &est_H_prime, &est_p_vec, &est_g_prime_vec, &est_h_vec, &est_g_vec_append, &est_h_vec_append, &est_C_vec_mut, &est_P_vec, &est_H_vec);
      assert!(result.is_ok());
      ver_proof_end = PreciseTime::now();
      total_ver_proof_duration += (ver_proof_start.to(ver_proof_end)).num_milliseconds();
    }

    let sample_gen_time = total_gen_proof_duration as f64/(1000.0*num_iter as f64);
    let sample_ver_time = total_ver_proof_duration as f64/(1000.0*num_iter as f64);
    let ratio = N/est_N;
    println!("Max estimated time: {:?}", (num_iter as f64) * ratio * (sample_gen_time + sample_ver_time));

    // Actual simulation
    let (G, H, Gt, H_prime, p_vec, g_prime_vec, h_vec, g_vec_append, h_vec_append, C_vec_mut, P_vec, H_vec, E_vec, x_vec, gamma) = MProvePlus::gen_params(n, s);

    total_gen_proof_duration = 0;
    total_ver_proof_duration = 0;
    let sim_start = PreciseTime::now();

    for _i in 0..num_iter {        

        gen_proof_start = PreciseTime::now();
        mprove_plus_proof = MProvePlus::prove(&G, &H, &Gt, &H_prime, &p_vec, &g_prime_vec, &h_vec, &g_vec_append, &h_vec_append, &C_vec_mut, &P_vec, &H_vec, &E_vec, &x_vec, &gamma);
        gen_proof_end = PreciseTime::now();
        total_gen_proof_duration += (gen_proof_start.to(gen_proof_end)).num_milliseconds();
  
        ver_proof_start = PreciseTime::now();
        result = mprove_plus_proof.verify(&G, &H, &Gt, &H_prime, &p_vec, &g_prime_vec, &h_vec, &g_vec_append, &h_vec_append, &C_vec_mut, &P_vec, &H_vec);
        assert!(result.is_ok());
        ver_proof_end = PreciseTime::now();
        total_ver_proof_duration += (ver_proof_start.to(ver_proof_end)).num_milliseconds();
    }
  
    let sim_end = PreciseTime::now();
    println!("Total simulation time = {:?}", sim_start.to(sim_end));

    println!("Average proof generation time = {:?}",
      (total_gen_proof_duration as f64/(1000.0*num_iter as f64)));
    println!("Average proof verification time = {:?}",
      (total_ver_proof_duration as f64/(1000.0*num_iter as f64)));

}
