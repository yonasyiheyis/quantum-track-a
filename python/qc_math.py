import numpy as np

def mag2(z: complex) -> float:
    """Return |z|^2 for a complex number z."""
    # |z|^2 = z * conj(z)
    return float(np.real(z * np.conjugate(z)))

def normalize(vec) -> np.ndarray:
    """
    Normalize a complex vector so that sum(|v_i|^2) == 1.
    Raises ValueError if the vector is all zeros.
    """
    v = np.asarray(vec, dtype=np.complex128)
    norm2 = float(np.sum(np.abs(v) ** 2))
    if norm2 == 0.0:
        raise ValueError("Cannot normalize the zero vector.")
    return v / np.sqrt(norm2)

def probs(vec) -> np.ndarray:
    """Return probabilities from amplitudes: normalize first, then |amp|^2."""
    v = normalize(vec)
    return (np.abs(v) ** 2).astype(np.float64)