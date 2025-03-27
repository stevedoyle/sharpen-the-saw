import pandas as pd


def fix_names(users: pd.DataFrame) -> pd.DataFrame:
    """
    Capitalizes the first letter of each name in the 'name' column of the given DataFrame
    and sorts the DataFrame by the 'user_id' column.

    Args:
        users (pd.DataFrame): A DataFrame containing at least the columns 'name' and 'user_id'.

    Returns:
        pd.DataFrame: A new DataFrame with the 'name' column capitalized and sorted by 'user_id'.
    """
    users["name"] = users["name"].str.capitalize()
    return users.sort_values("user_id")


class TestFixNames:
    def test_fix_names(self):
        data = [[1, "aLice"], [2, "bOB"]]
        users = pd.DataFrame(data, columns=["user_id", "name"]).astype(
            {"user_id": "Int64", "name": "object"}
        )

        df = fix_names(users)
        assert df.shape == (2, 2)
        assert (df.columns == ["user_id", "name"]).all()
        assert df["name"].tolist() == ["Alice", "Bob"]
