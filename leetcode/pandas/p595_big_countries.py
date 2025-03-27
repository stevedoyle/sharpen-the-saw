import pandas as pd


def big_countries(world: pd.DataFrame) -> pd.DataFrame:
    """
    Filters a DataFrame of countries to return only those with an area of at least
    3,000,000 square kilometers or a population of at least 25,000,000.

    Args:
        world (pd.DataFrame): A DataFrame containing information about countries
            with at least the following columns:
            - "name" (str): The name of the country.
            - "area" (float or int): The area of the country in square kilometers.
            - "population" (float or int): The population of the country.

    Returns:
        pd.DataFrame: A DataFrame containing the filtered countries with the
        following columns:
            - "name" (str): The name of the country.
            - "area" (float or int): The area of the country in square kilometers.
            - "population" (float or int): The population of the country.
    """
    df = world[(world.area >= 3000000) | (world.population >= 25000000)]
    df = df[["name", "area", "population"]]
    return df


class TestBigCountries:
    def test_ex1(self):
        data = [
            ["Afghanistan", "Asia", 652230, 25500100, 20343000000],
            ["Albania", "Europe", 28748, 2831741, 12960000000],
            ["Algeria", "Africa", 2381741, 37100000, 188681000000],
            ["Andorra", "Europe", 468, 78115, 3712000000],
            ["Angola", "Africa", 1246700, 20609294, 100990000000],
        ]
        world = pd.DataFrame(
            data, columns=["name", "continent", "area", "population", "gdp"]
        ).astype(
            {
                "name": "object",
                "continent": "object",
                "area": "Int64",
                "population": "Int64",
                "gdp": "Int64",
            }
        )

        df = big_countries(world)
        assert df.shape == (2, 3)
        assert (df.name == ["Afghanistan", "Algeria"]).all()
        assert (df.columns == ["name", "area", "population"]).all()
