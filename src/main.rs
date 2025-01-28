use rand::prelude::*;
use rand_distr::Normal;

struct LiborMarketModelParams {
    forward_rates: Vec<f32>,
    volatility: Vec<f32>,
    correlations: Vec<Vec<f32>>,
    dt: f32,
    T: f32,
}

fn generate_correlation_matrix(n: usize, decay_factor: f32) -> Vec<Vec<f32>> {
    let mut matrix = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                matrix[i][j] = 1.0;
            } else {
                let distance = (i as isize - j as isize).abs() as f32;
                matrix[i][j] = (-decay_factor * distance).exp();
            }
        }
    }
    matrix
}

fn standard_normal_generator(
    n_instruments: usize,
    n_steps: usize,
    n_sims: usize,
    mean: f32,
    std_dev: f32,
) -> Vec<Vec<Vec<f32>>> {
    let mut rng = thread_rng();
    let normal = Normal::new(mean, std_dev).unwrap();

    (0..n_instruments)
        .map(|_| {
            (0..n_steps)
                .map(|_| (0..n_sims).map(|_| normal.sample(&mut rng)).collect())
                .collect()
        })
        .collect()
}

fn get_vol_sim_matrix(
    vols: &[f32],
    n_steps: usize,
) -> Vec<Vec<f32>> {
    
    (0..n_steps).map(|_| vols.to_vec()).collect()
}

fn main() {
    let n_instruments: usize = 6;
    let n_sims: usize = 10_000;

    let lmm_params = LiborMarketModelParams {
        forward_rates: vec![0.04; n_instruments],
        volatility: vec![0.1; n_instruments],
        correlations: generate_correlation_matrix(n_instruments, 0.01),
        dt: 1.0 / 252.0,
        T: 1.0,
    };

    let n_steps: usize = (lmm_params.T / lmm_params.dt) as usize;

    let st_norm_vec: Vec<Vec<Vec<f32>>> = standard_normal_generator(n_instruments, n_steps, n_sims, 0.0, 1.0);

    let vol_matrix: Vec<Vec<f32>> = get_vol_sim_matrix(&lmm_params.volatility, n_steps);

    let mut forward_simulation = vec![vec![vec![0.0; n_instruments]; n_sims]; n_steps];

    for i_sim in 0..n_sims {
        forward_simulation[0][i_sim].copy_from_slice(&lmm_params.forward_rates);
    }

    for i_timestep in 1..n_steps {
        for i_sim in 0..n_sims {
            for i_instrument in 0..n_instruments {

                let df_t: f32 = forward_simulation[i_timestep - 1][i_sim][i_instrument] * vol_matrix[i_timestep][i_instrument] * st_norm_vec[i_instrument][i_timestep][i_sim];
                
                forward_simulation[i_timestep][i_sim][i_instrument] = forward_simulation[i_timestep - 1][i_sim][i_instrument] + df_t;
            }
        }
    }
}