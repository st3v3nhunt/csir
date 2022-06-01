# CLI Stock Info Retriever (CSIR)

> Retrieve stock info with CLI tool

Simple CLI tool to retrieve stock information from
[Financial Modeling Prep API](https://site.financialmodelingprep.com/developer)
(FMP).

## Usage

An environment variable with the name `API_KEY` and the value of the `apikey`
for a plan at FMP needs to be set within the environment where
CSIR is running. This can be done by setting the env var directly in the
environment or by having a `.env` file in the directory (or a parent directory)
where CSIR will run.
