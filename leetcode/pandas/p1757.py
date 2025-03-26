import pandas as pd


def find_products(products: pd.DataFrame) -> pd.DataFrame:
    df = products[(products.low_fats == "Y") & (products.recyclable == "Y")]
    return df[["product_id"]]


data = [
    ["0", "Y", "N"],
    ["1", "Y", "Y"],
    ["2", "N", "Y"],
    ["3", "Y", "Y"],
    ["4", "N", "N"],
]
products = pd.DataFrame(data, columns=["product_id", "low_fats", "recyclable"]).astype(
    {"product_id": "int64", "low_fats": "category", "recyclable": "category"}
)


class TestFindProducts:
    def test_find_products(self):
        df = find_products(products)
        assert df.shape == (2, 1)
        assert df.columns == ["product_id"]
        assert df["product_id"].tolist() == [1, 3]
