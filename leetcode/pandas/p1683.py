import pandas as pd


def invalid_tweets(tweets: pd.DataFrame) -> pd.DataFrame:
    """
    Filters a DataFrame of tweets to return only the IDs of invalid tweets.

    A tweet is considered invalid if its content exceeds 15 characters in length.

    Args:
        tweets (pd.DataFrame): A DataFrame containing tweet data. It must have
            the following columns:
            - 'tweet_id': The unique identifier for each tweet.
            - 'content': The text content of the tweet.

    Returns:
        pd.DataFrame: A DataFrame containing a single column 'tweet_id' with
        the IDs of tweets that are invalid.
    """
    df = tweets[tweets.content.str.len() > 15]
    df = df[["tweet_id"]]
    return df


class TestInvalidTweets:
    def test_invalid_tweets(self):
        data = [[1, "Let us Code"], [2, "More than fifteen chars are here!"]]
        tweets = pd.DataFrame(data, columns=["tweet_id", "content"]).astype(
            {"tweet_id": "Int64", "content": "object"}
        )

        df = invalid_tweets(tweets)
        assert df.shape == (1, 1)
        assert df.columns == ["tweet_id"]
        assert df["tweet_id"].tolist() == [2]
