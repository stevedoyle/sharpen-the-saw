import numpy as np
import pandas as pd
from datetime import datetime, timedelta

# Set random seed for reproducibility
np.random.seed(42)


def generate_sample_dataset():
    """Generate a comprehensive sample dataset for data analysis practice."""

    n_samples = 1000

    # Generate various types of data
    data = {
        # Normal distribution - ages
        "age": np.random.normal(35, 12, n_samples).astype(int),
        # Exponential distribution - income (skewed)
        "income": np.random.exponential(50000, n_samples),
        # Uniform distribution - test scores
        "test_score": np.random.uniform(0, 100, n_samples),
        # Binomial distribution - number of purchases
        "purchases_per_month": np.random.binomial(20, 0.3, n_samples),
        # Categorical data
        "department": np.random.choice(
            ["Sales", "Engineering", "Marketing", "HR", "Finance"],
            n_samples,
            p=[0.3, 0.25, 0.2, 0.15, 0.1],
        ),
        # Boolean data
        "has_degree": np.random.choice([True, False], n_samples, p=[0.7, 0.3]),
        # Discrete values - years of experience
        "experience_years": np.random.poisson(8, n_samples),
        # Response time (log-normal distribution)
        "response_time_ms": np.random.lognormal(3, 1, n_samples),
        # Customer satisfaction (beta distribution, bounded 1-10)
        "satisfaction_rating": np.random.beta(2, 1, n_samples) * 9 + 1,
    }

    # Create DataFrame
    df = pd.DataFrame(data)

    # Clean up data to make it more realistic
    df["age"] = np.clip(df["age"], 18, 80)
    df["income"] = np.clip(df["income"], 20000, 300000)
    df["test_score"] = np.round(df["test_score"], 1)
    df["experience_years"] = np.clip(df["experience_years"], 0, 40)
    df["response_time_ms"] = np.round(df["response_time_ms"], 2)
    df["satisfaction_rating"] = np.round(df["satisfaction_rating"], 1)
    df["income"] = np.round(df["income"], 2)

    # Add some correlation - higher experience tends to mean higher income
    experience_bonus = df["experience_years"] * 2000 + np.random.normal(
        0, 5000, n_samples
    )
    df["income"] = df["income"] + experience_bonus
    df["income"] = np.clip(df["income"], 20000, 400000)
    df["income"] = np.round(df["income"], 2)

    # Add employee ID
    df["employee_id"] = range(1, n_samples + 1)

    return df


if __name__ == "__main__":
    # Generate the dataset
    dataset = generate_sample_dataset()

    # Save to CSV
    dataset.to_csv("sample_data.csv", index=False)

    # Display basic info about the dataset
    print("Sample Dataset Generated!")
    print(f"Shape: {dataset.shape}")
    print("\nColumn Types:")
    print(dataset.dtypes)
    print("\nFirst 5 rows:")
    print(dataset.head())
    print("\nBasic Statistics:")
    print(dataset.describe())
