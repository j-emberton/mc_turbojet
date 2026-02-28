use pyo3::prelude::*;

const GAMMA:f64 = 1.4;
const CP: f64 = 1004.5;
const LHV: f64 = 43e6;
const ETA_B: f64 = 0.98;
const ETA_M: f64 = 0.99;

const TA: f64 = 288.15;
const PA: f64 = 101325.0;


/// Evaluates the performance of a simple turbojet engine based on input parameters.
#[pyfunction]
fn evaluate_performance(
    pi_c: f64, //Compressor pressure ratio
    tt4: f64, // Turbine inlet temperature
    eta_c: f64, // Compressor efficiency
    eta_t: f64, // Turbine efficiency
    eta_n: f64, // Nozzle efficiency
    pi_b: f64, // Combustor pressure ratio
) -> PyResult<(f64, f64)> {
    if pi_c <= 1.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Compressor pressure ratio must be greater than 1."
        ));
    }
    // inlet conditions
    let tt2 = TA;
    let pt2 = PA;

    // compressor
    let tt3 = tt2 * (pi_c.powf((GAMMA -1.0) / GAMMA));
    let tt3 = tt2 + (tt3 - tt2) / eta_c;
    let pt3 = pt2 * pi_c;
    let wc = CP * (tt3 - tt2);

    // combustor
    let pt4 = pt3 * pi_b;

    let f =CP * (tt4 - tt3) / (ETA_B * LHV - CP*tt4);

    if f < 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Fuel-air ratio is negative. Check input parameters."
        ));
    }

    // turbine
    let wt = wc / ETA_M;
    let tt5 = tt4 - wt / CP;

    if tt5 < 0.0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Turbine exit temperature is negative. Check input parameters."
        ))
    }

    let tt5s = tt4 - wt / (CP * eta_t);
    let pt5 = pt4 * (tt5s / tt4).powf(GAMMA / (GAMMA - 1.0));

    if pt5 <= PA {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Turbine exit pressure is below ambient. Check input parameters."
        ))
    }

    // nozzle
    let te_s = tt5*(PA / pt5).powf((GAMMA - 1.0) / GAMMA);
    let ve_s = (2.0 * CP * (tt5 - te_s)).sqrt();
    let ve = eta_n.sqrt() * ve_s;

    let specific_thrust = (1.0 + f)*ve;

    Ok((ve, specific_thrust))
    }


/// A Python module implemented in Rust.
#[pymodule]
fn mc_turbojet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate_performance, m)?)?;
    Ok(())
}
