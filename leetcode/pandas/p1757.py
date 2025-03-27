import pandas as pd


def find_products(products: pd.DataFrame) -> pd.DataFrame:
    """
    Filters the products DataFrame to find products that are both low fat and recyclable.

    Args:
        products (pd.DataFrame): A DataFrame containing product information with at least
            the following columns:
            - 'low_fats' (str): Indicates if the product is low fat ('Y' or 'N').
            - 'recyclable' (str): Indicates if the product is recyclable ('Y' or 'N').
            - 'product_id' (int): The unique identifier for the product.

    Returns:
        pd.DataFrame: A DataFrame containing only the 'product_id' column for products
        that are both low fat and recyclable.
    """
    df = products[(products.low_fats == "Y") & (products.recyclable == "Y")]
    return df[["product_id"]]


class TestFindProducts:
    def test_find_products(self):
        data = [
            ["0", "Y", "N"],
            ["1", "Y", "Y"],
            ["2", "N", "Y"],
            ["3", "Y", "Y"],
            ["4", "N", "N"],
        ]
        products = pd.DataFrame(
            data, columns=["product_id", "low_fats", "recyclable"]
        ).astype(
            {"product_id": "int64", "low_fats": "category", "recyclable": "category"}
        )

        df = find_products(products)
        assert df.shape == (2, 1)
        assert df.columns == ["product_id"]
        assert df["product_id"].tolist() == [1, 3]
