import pytest

import mc_turbojet as tj


def test_evaluate_runs():
    ve, thrust = tj.evaluate_performance(20.0, 1600.0, 0.9, 0.92, 0.95, 0.95)
    assert ve > 0
    assert thrust > 0


def test_deterministic():
    r1 = tj.evaluate_performance(20.0, 1600.0, 0.9, 0.92, 0.95, 0.95)
    r2 = tj.evaluate_performance(20.0, 1600.0, 0.9, 0.92, 0.95, 0.95)
    assert r1 == r2


def test_thrust_increases_with_tt4():
    _, thrust_low = tj.evaluate_performance(20.0, 1500.0, 0.9, 0.92, 0.95, 0.95)
    _, thrust_high = tj.evaluate_performance(20.0, 1700.0, 0.9, 0.92, 0.95, 0.95)

    assert thrust_high > thrust_low


def test_invalid_pressure_ratio():
    with pytest.raises(ValueError):
        tj.evaluate_performance(1.0, 1600.0, 0.9, 0.92, 0.95, 0.95)
