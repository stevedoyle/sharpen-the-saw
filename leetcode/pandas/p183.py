import pandas as pd


def find_customers(customers: pd.DataFrame, orders: pd.DataFrame) -> pd.DataFrame:
    df = customers[~customers["id"].isin(orders["customerId"])]
    df = df[["name"]].rename(columns={"name": "Customers"})
    return df


class TestFindCustomers:
    def test_find_customers(self):
        cust_data = [[1, "Joe"], [2, "Henry"], [3, "Sam"], [4, "Max"]]
        customers = pd.DataFrame(cust_data, columns=["id", "name"]).astype(
            {"id": "Int64", "name": "object"}
        )
        order_data = [[1, 3], [2, 1]]
        orders = pd.DataFrame(order_data, columns=["id", "customerId"]).astype(
            {"id": "Int64", "customerId": "Int64"}
        )

        df = find_customers(customers, orders)
        assert df.shape == (2, 1)
        assert df["Customers"].tolist() == ["Henry", "Max"]
