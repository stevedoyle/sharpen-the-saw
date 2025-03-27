import pandas as pd


def calculate_special_bonus(employees: pd.DataFrame) -> pd.DataFrame:
    """
    Calculate the special bonus for employees based on specific conditions.

    The bonus is equal to the employee's salary if:
    - The employee's ID is odd, and
    - The employee's name does not start with the letter 'M'.

    Otherwise, the bonus is 0.

    Args:
        employees (pd.DataFrame): A DataFrame containing employee information with the following columns:
            - "employee_id" (int): The unique ID of the employee.
            - "name" (str): The name of the employee.
            - "salary" (int): The salary of the employee.

    Returns:
        pd.DataFrame: A DataFrame with two columns:
            - "employee_id" (int): The unique ID of the employee.
            - "bonus" (int): The calculated bonus for the employee, sorted by "employee_id".
    """
    employees["bonus"] = employees.apply(
        lambda x: (
            x["salary"]
            if x["employee_id"] % 2 != 0 and not x["name"].startswith("M")
            else 0
        ),
        axis=1,
    )
    return employees[["employee_id", "bonus"]].sort_values("employee_id")


class TestCalculateSpecialBonus:
    def test_calculate_special_bonus(self):
        data = [
            [2, "Meir", 3000],
            [3, "Michael", 3800],
            [9, "Kannon", 7700],
            [8, "Juan", 6100],
            [7, "Addilyn", 7400],
        ]
        employees = pd.DataFrame(
            data, columns=["employee_id", "name", "salary"]
        ).astype({"employee_id": "int64", "name": "object", "salary": "int64"})

        df = calculate_special_bonus(employees)
        assert df.shape == (5, 2)
        assert (df.columns == ["employee_id", "bonus"]).all()
        assert df["employee_id"].tolist() == [2, 3, 7, 8, 9]
        assert df["bonus"].tolist() == [0, 0, 7400, 0, 7700]
