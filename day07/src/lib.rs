use permutohedron::Heap;

use crate::amplifier::Amplifier;

mod amplifier;

pub fn compute_best_phase_cfg(memory: Vec<i32>) -> (Vec<u8>, i32) {
    let mut phases = (0..=4).collect::<Vec<_>>();
    let mut phases_permutations = Heap::new(&mut phases).collect::<Vec<_>>();
    phases_permutations.sort();

    phases_permutations
        .into_iter()
        .map(|phase| {
            let signal = amp_seq(memory.clone(), &phase);
            (phase, signal)
        })
        .max_by(|(_, signal1), (_, signal2)| signal1.cmp(signal2))
        .unwrap()
}

fn amp_seq(memory: Vec<i32>, phases: &[u8]) -> i32 {
    let mut signal = 0;

    for &phase in phases {
        let mut amp = Amplifier::new(memory.clone(), phase);
        amp.push_signal(signal);
        amp.wait_termination();

        signal = amp
            .pop_signal()
            .expect("Amplifier didn't produced a value !");
    }

    signal
}

pub fn compute_best_phase_cfg_lo(memory: Vec<i32>) -> (Vec<u8>, i32) {
    let mut phases = (5..=9).collect::<Vec<_>>();
    let mut phases_permutations = Heap::new(&mut phases).collect::<Vec<_>>();
    phases_permutations.sort();

    phases_permutations
        .into_iter()
        .map(|phase| {
            let signal = amp_seq_lo(memory.clone(), &phase);
            (phase, signal)
        })
        .max_by(|(_, signal1), (_, signal2)| signal1.cmp(signal2))
        .unwrap()
}

fn amp_seq_lo(memory: Vec<i32>, phases: &[u8]) -> i32 {
    let mut amps = Vec::new();
    for &phase in phases {
        amps.push(Amplifier::new(memory.clone(), phase));
    }

    let mut signal = 0;
    loop {
        // Run the first amp and check for an halt
        if let Some(val) = amplifier_round(&mut amps[0], signal) {
            // Everything good
            signal = val;
        } else if amps[0].halted() {
            // Ok everything is normal
            break;
        } else {
            // Fuck fuck fuck
            panic!("The first amplifier didn't produced a value !!");
        }

        // Run 4 last amps, they can't fail
        for amp in amps.iter_mut().skip(1) {
            signal =
                amplifier_round(amp, signal).expect("Middle amplifier didn't produced a value !");
        }
    }

    // Graceful shutdown
    for amp in amps.iter_mut() {
        amp.wait_termination();
    }

    signal
}

fn amplifier_round(amp: &mut Amplifier, signal: i32) -> Option<i32> {
    amp.push_signal(signal);
    amp.resume();

    amp.pop_signal()
}
