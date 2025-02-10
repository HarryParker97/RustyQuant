use rand::prelude::*;
use rand_distr::Normal;
use nalgebra::DMatrix;


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

struct LiborMarketModel {
    forward_rates: Vec<f32>,
    forward_tau: Vec<f32>,
    volatility: Vec<f32>,
    correlations: Vec<Vec<f32>>,
    numeraire_index: usize,
    dt: f32,
    T: f32,
}

impl LiborMarketModel {
    
    fn standard_normal_generator(
        &self,
        n_instruments: usize,
        n_steps: usize,
        n_sims: usize,
        mean: f32,
        std_dev: f32,
    ) -> Vec<Vec<Vec<f32>>> {
        
        let mut rng = thread_rng();
        let normal = Normal::new(mean, std_dev).unwrap();
    
        let corr_d_matrix = DMatrix::from_vec(
            self.correlations.len(),
            self.correlations[0].len(),
            self.correlations.iter().flatten().copied().collect(),
        );
    
        let cholesky = corr_d_matrix.cholesky().expect("Cholesky decomposition failed");
        let l_matrix = cholesky.l();
    
        (0..n_steps)
            .map(|_| {
                (0..n_sims)
                    .map(|_| {
                        let norm_vec: Vec<f32> = (0..n_instruments).map(|_| normal.sample(&mut rng)).collect();
                        let norm_matrix = DMatrix::from_column_slice(n_instruments, 1, &norm_vec);
                        let correlated = &l_matrix * norm_matrix;
                        correlated.column(0).iter().copied().collect()
                    })
                    .collect()
            })
            .collect()
    }
    
    fn get_vol_sim_matrix(
        &self,
        n_steps: usize,
    ) -> Vec<Vec<f32>> {
        
        (0..n_steps).map(|_| self.volatility.iter().map(|&v| v *self.dt.sqrt()).collect()).collect()
    }

    fn simulate(&self, n_instruments: usize, n_sims: usize) -> Vec<Vec<Vec<f32>>> {
        let n_steps: usize = (self.T / self.dt) as usize;

        let st_norm_vec: Vec<Vec<Vec<f32>>> = self.standard_normal_generator(n_instruments, n_steps, n_sims, 0.0, 1.0);
    
        let vol_matrix: Vec<Vec<f32>> = self.get_vol_sim_matrix(n_steps);
        

        let mut forward_simulation = vec![vec![vec![0.0; n_instruments]; n_sims]; n_steps];
        for i_sim in 0..n_sims {
            forward_simulation[0][i_sim].copy_from_slice(&self.forward_rates);
        }
    
        for i_timestep in 1..n_steps {
            for i_sim in 0..n_sims {
                for i_instrument in 0..n_instruments {
                    
                    let drift: f32 = match i_instrument {
                        _ if i_instrument < self.numeraire_index => {
                            let mut drift: f32 = 0.0;
                            for j_instrument in 0..n_instruments {
                                drift += (
                                    self.correlations[i_instrument][j_instrument] 
                                    * vol_matrix[i_timestep - 1][j_instrument] 
                                    * self.forward_tau[j_instrument] 
                                    * self.forward_rates[j_instrument]
                                ) / (
                                    1.0_f32 + self.forward_tau[j_instrument] * self.forward_rates[j_instrument]
                                )
                            }
                            drift *  self.forward_rates[i_instrument]  * self.volatility[i_instrument]
                        },
                        _ if i_instrument == self.numeraire_index => 0.0,
                        _ if i_instrument > self.numeraire_index => {
                            let mut drift: f32 = 0.0;
                            for j_instrument in 0..n_instruments {
                                drift -= (
                                    self.correlations[i_instrument][j_instrument]
                                     * vol_matrix[i_timestep - 1][j_instrument] 
                                     * self.forward_tau[j_instrument]
                                     * self.forward_rates[j_instrument]
                                ) / (
                                    1.0_f32 + self.forward_tau[j_instrument] * self.forward_rates[j_instrument]
                                )
                            }
                            drift *  self.forward_rates[i_instrument] * self.volatility[i_instrument]
                        }
                        _ => unreachable!(),
                    };
    
                    let diffusion: f32 = forward_simulation[i_timestep - 1][i_sim][i_instrument] * vol_matrix[i_timestep][i_instrument] * st_norm_vec[i_timestep][i_sim][i_instrument];
                    
                    forward_simulation[i_timestep][i_sim][i_instrument] = forward_simulation[i_timestep - 1][i_sim][i_instrument] + diffusion + drift * self.dt;
                }
            }
        }
        forward_simulation
    }
}

fn main() {
    let n_instruments: usize = 6;
    let n_sims: usize = 10_000;

    let lmm_params = LiborMarketModel {
        forward_rates: vec![0.04; n_instruments],
        forward_tau: vec![0.5; n_instruments],
        volatility: vec![0.1; n_instruments],
        correlations: generate_correlation_matrix(n_instruments, 0.01),
        numeraire_index: 3,
        dt: 1.0 / 252.0,
        T: 1.0,
    };

    let forward_simulation: Vec<Vec<Vec<f32>>> = lmm_params.simulate(n_instruments, n_sims);

    let mc_fwd_rates: Vec<f32> = (0..n_instruments)
    .map(|i_instrument| {
        forward_simulation
            .last()
            .unwrap()
            .iter()
            .map(|sim| sim[i_instrument])
            .sum::<f32>() / n_sims as f32
    })
    .collect();

    for mc_fwd_rate in mc_fwd_rates {
        println!("{}", mc_fwd_rate);
    }
}