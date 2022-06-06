# CLI Stock Info Retriever (CSIR)

> Retrieve stock info with CLI tool

Simple CLI tool to retrieve stock information from
[Financial Modeling Prep API](https://site.financialmodelingprep.com/developer)
(FMP).

## Usage

There are a number of environment variables required to be set for the
successful functioning of the tool, see below for details.

Env vars can be set directly in the environment where the tool is running or by
having a `.env` file in the directory (or a parent directory).

### Env vars

| Name                      | Description                                                                                            |
| ----                      | -----------                                                                                            |
| `API_KEY`                 | apiKey for FMP, get one [here](https://site.financialmodelingprep.com/developer/docs/pricing/)         |
| `REMAINING_CALLS_API_URL` | URL for API used to retrieve the number of calls remaining for the `API_KEY` within the 24 hour period |

## To-do

### General / QOL improvements

- [ ] Add tests
- [ ] Check test coverage for some minimum
- [ ] Add CI (using GitHub Actions)
- [ ] Create release when merged to `main`
- [ ] Consider an 'admin' subcommand where commands like get-remaining-calls
  can be grouped
- [ ] Consider validating symbols - check against a search
- [ ] Improve logging - log by level, set level by CLI option
- [ ] Print out full response when specific setting is provided

### Features

- [ ] Retrieve historical data
- [ ] Visualise historical data

## Done

- [x] Improve file structure
- [x] Get price change for symbols
- [x] Get remaining API calls
