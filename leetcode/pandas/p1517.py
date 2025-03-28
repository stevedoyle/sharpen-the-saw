import pandas as pd


def valid_emails(users: pd.DataFrame) -> pd.DataFrame:
    """
    Filters a DataFrame to return rows where the 'mail' column contains valid email addresses
    that belong to the 'leetcode.com' domain.

    A valid email address must:
    - Start with an alphabetic character (a-z, A-Z).
    - Be followed by zero or more alphanumeric characters, dots (.), underscores (_), or hyphens (-).
    - End with '@leetcode.com'.

    Args:
        users (pd.DataFrame): A DataFrame containing a 'mail' column with email addresses.

    Returns:
        pd.DataFrame: A filtered DataFrame containing only rows with valid 'leetcode.com' email addresses.
    """
    df = users[users.mail.str.contains(r"^[a-zA-Z][a-zA-Z0-9._-]*@leetcode\.com$")]
    return df


class TestValidEmails:
    def test_valid_emails(self):

        data = [
            [1, "Winston", "winston@leetcode.com"],
            [2, "Jonathan", "jonathanisgreat"],
            [3, "Annabelle", "bella-@leetcode.com"],
            [4, "Sally", "sally.come@leetcode.com"],
            [5, "Marwan", "quarz#2020@leetcode.com"],
            [6, "David", "david69@gmail.com"],
            [7, "Shapiro", ".shapo@leetcode.com"],
        ]
        users = pd.DataFrame(data, columns=["user_id", "name", "mail"]).astype(
            {"user_id": "int64", "name": "object", "mail": "object"}
        )

        df = valid_emails(users)
        assert df.shape == (3, 3)
        assert (df.columns == ["user_id", "name", "mail"]).all()
        assert df.user_id.tolist() == [1, 3, 4]
