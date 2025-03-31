import pandas as pd


def find_patients(patients: pd.DataFrame) -> pd.DataFrame:
    """
    Filters a DataFrame of patients to find those with the condition "DIAB1".

    The function searches for the substring "DIAB1" in the "conditions" column
    of the input DataFrame. It matches "DIAB1" as a whole word, either at the
    start of the string or preceded by a space.

    Args:
        patients (pd.DataFrame): A DataFrame containing patient data, where
            one of the columns is named "conditions" and contains condition
            descriptions as strings.

    Returns:
        pd.DataFrame: A filtered DataFrame containing only the rows where the
        "conditions" column contains the substring "DIAB1".
    """
    # Filter patients with "DIAB1" in their conditions
    # The regex pattern "(?:^|\s)DIAB1" matches "DIAB1" at the start of the string
    # or preceded by a space. The "(?:" is a non-capturing group.
    return patients[patients["conditions"].str.contains(r"(?:^|\s)DIAB1", regex=True)]


class TestFindPatients:
    def test_find_patients(self):
        data = [
            [1, "Daniel", "YFEV COUGH"],
            [2, "Alice", ""],
            [3, "Bob", "DIAB100 MYOP"],
            [4, "George", "ACNE DIAB100"],
            [5, "Alain", "DIAB201"],
        ]
        patients = pd.DataFrame(
            data, columns=["patient_id", "patient_name", "conditions"]
        ).astype(
            {"patient_id": "int64", "patient_name": "object", "conditions": "object"}
        )

        df = find_patients(patients)
        assert df.shape == (2, 3)
        assert (df.columns == ["patient_id", "patient_name", "conditions"]).all()
        assert df["patient_name"].tolist() == ["Bob", "George"]
        assert df["conditions"].tolist() == ["DIAB100 MYOP", "ACNE DIAB100"]
        assert df["patient_id"].tolist() == [3, 4]
