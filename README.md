# Donation Transparency

## Premise

Donation data concering the Australian election is publically available here: [AEC Detailed Receipts](https://transparency.aec.gov.au/AnnualDetailedReceipts). However, the website I find is extremely hard to parse and does not offer a basic understanding of, how much did party A, get donated in financial year B.

This service parses the detailed receipts data and populates a Postgres database for you, and provides helper functions for querying the data. Currently I'm publically hosting a website where you can view the data with my own frontend built on top of this service (I'm bad at frontend engineering so won't be making this code public soz).

## Data Visualisation

If you want to see the data check out my website here: [Anby Katz Blog](https://anbykatz.me/projects/donation-transparency/party).

## Running Yourself

Just add this crate with your own rust application and away you go. You can populate your own postgres database by doing the following. Get the exported .xlsx file from here: [AEC Detailed Receipts](https://transparency.aec.gov.au/AnnualDetailedReceipts, dump this file like so `donation-transparency/resources/DetailedReceipts.csv`. Now run the following.

```bash
cd doantion-transparency/resources
psql -d DATABASE_URL

# Will create the tables and populate them for you
\i donation.sql
```

Currently I'm using [Actix Web](https://actix.rs/) for my API endpoints which I would HIGHLY recommend. Pick you're favourite frontend framework or just use native rust and you're absolutely COOKING.
