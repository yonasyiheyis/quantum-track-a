import numpy as np
import pytest
from qc_math import mag2, normalize, probs

def test_mag2_3_4i():
    assert mag2(3 + 4j) == 25.0

def test_mag2_1_minus_i():
    assert mag2(1 - 1j) == 2.0

def test_probs_equal_superposition():
    p = probs([1+0j, 1+0j])
    assert np.allclose(p, [0.5, 0.5])

def test_probs_single_state():
    p = probs([1+1j, 0+0j])
    assert np.allclose(p, [1.0, 0.0])

def test_probs_sum_to_one():
    p = probs([1+2j, 3-4j])
    assert abs(float(np.sum(p)) - 1.0) < 1e-12

def test_normalize_zero_vector_raises():
    with pytest.raises(ValueError):
        normalize([0+0j, 0+0j])