import pandas as pd


def article_views(views: pd.DataFrame) -> pd.DataFrame:
    """
    Filters and processes a DataFrame of article views to identify unique authors
    who have viewed their own articles.

    Args:
        views (pd.DataFrame): A DataFrame containing article view data with the following columns:
            - "author_id": The ID of the author of the article.
            - "viewer_id": The ID of the user who viewed the article.

    Returns:
        pd.DataFrame: A DataFrame with a single column "id" containing the unique IDs
        of authors who have viewed their own articles, sorted in ascending order.
    """
    df = views[views["author_id"] == views["viewer_id"]]
    df = df.drop_duplicates(subset=["author_id"])
    df = df[["author_id"]].rename(columns={"author_id": "id"})
    df.sort_values("id", inplace=True)
    return df


class TestArticleViews:
    def test_article_views(self):
        data = [
            [1, 3, 5, "2019-08-01"],
            [1, 3, 6, "2019-08-02"],
            [2, 7, 7, "2019-08-01"],
            [2, 7, 6, "2019-08-02"],
            [4, 7, 1, "2019-07-22"],
            [3, 4, 4, "2019-07-21"],
            [3, 4, 4, "2019-07-21"],
        ]

        views = pd.DataFrame(
            data, columns=["article_id", "author_id", "viewer_id", "view_date"]
        ).astype(
            {
                "article_id": "Int64",
                "author_id": "Int64",
                "viewer_id": "Int64",
                "view_date": "datetime64[ns]",
            }
        )

        df = article_views(views)
        assert df.shape == (2, 1)
        assert df.columns == ["id"]
        print(df)
        assert df.id.to_list() == [4, 7]
