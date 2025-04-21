DROP TABLE IF EXISTS Donar CASCADE;
DROP TABLE IF EXISTS Donation CASCADE;
DROP TABLE IF EXISTS Party CASCADE;
DROP TABLE IF EXISTS Branch CASCADE;
DROP TABLE IF EXISTS Industry CASCADE;

ALTER TABLE Branch DROP CONSTRAINT IF EXISTS FK_Party_TO_Branch;
ALTER TABLE Donar DROP CONSTRAINT IF EXISTS FK_Industry_TO_Donar;
ALTER TABLE Donation DROP CONSTRAINT IF EXISTS FK_Branch_TO_Donation;
ALTER TABLE Donation DROP CONSTRAINT IF EXISTS FK_Donar_TO_Donation;

CREATE TABLE Branch
(
  id       serial  NOT NULL,
  name     varchar NOT NULL,
  party_id integer,
  PRIMARY KEY (id)
);

CREATE TABLE Donar
(
  id          serial  NOT NULL,
  name        varchar NOT NULL,
  industry_id integer,
  PRIMARY KEY (id)
);

CREATE TABLE Donation
(
  id        serial  NOT NULL,
  year      varchar NOT NULL,
  amount    bigint  NOT NULL,
  branch_id integer  NOT NULL,
  donar_id  integer  NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE Industry
(
  id   serial  NOT NULL,
  name varchar NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE Party
(
  id   serial  NOT NULL,
  name varchar NOT NULL,
  PRIMARY KEY (id)
);

ALTER TABLE Branch
  ADD CONSTRAINT FK_Party_TO_Branch
    FOREIGN KEY (party_id)
    REFERENCES Party (id)
    ON DELETE CASCADE;

ALTER TABLE Donar
  ADD CONSTRAINT FK_Industry_TO_Donar
    FOREIGN KEY (industry_id)
    REFERENCES Industry (id)
    ON DELETE CASCADE;

ALTER TABLE Donation
  ADD CONSTRAINT FK_Branch_TO_Donation
    FOREIGN KEY (branch_id)
    REFERENCES Branch (id)
    ON DELETE CASCADE;

ALTER TABLE Donation
  ADD CONSTRAINT FK_Donar_TO_Donation
    FOREIGN KEY (donar_id)
    REFERENCES Donar (id)
    ON DELETE CASCADE;

ALTER TABLE Party
  ADD CONSTRAINT unique_party_name UNIQUE (name);

ALTER TABLE Branch
  ADD CONSTRAINT unique_branch_name UNIQUE (name);  

ALTER TABLE Donar
  ADD CONSTRAINT unique_donar_name UNIQUE (name);  

/* CSV inserts */
CREATE TEMP TABLE temp_receipt_data (
    "Financial Year" TEXT,
    "Return Type" TEXT,
    "Recipient Name" TEXT,
    "Received From" TEXT,
    "Receipt Type" TEXT,
    "Amount" BIGINT
);

CREATE TEMP TABLE temp_excluded_data (
    name TEXT
);

\copy temp_receipt_data FROM 'DetailedReceipts.csv' DELIMITER ',' CSV HEADER;
\copy temp_excluded_data FROM 'Excluded.csv' DELIMITER ',' CSV HEADER;
\copy Party(name) FROM 'Parties.csv' DELIMITER ',' CSV HEADER;

/* Insert to get IDs */
INSERT INTO Branch (name) 
  SELECT DISTINCT "Recipient Name" FROM temp_receipt_data
  WHERE "Return Type" IN ('Political Party Return', 'Member of HOR Return');
INSERT INTO Donar (name) 
  SELECT DISTINCT "Received From" FROM temp_receipt_data
  WHERE "Return Type" IN ('Political Party Return', 'Member of HOR Return');

/* This parties break the branch convention so need to be excluded i.e. contain breaking strings */
\set excluded_branchs (SELECT * FROM temp_excluded_data);

/* Associate the branches with the parties */
UPDATE Branch
SET party_id = Party.id
FROM Party
WHERE POSITION(LOWER(Party.name) IN LOWER(Branch.name)) > 0
  AND Branch.name NOT IN (SELECT * FROM temp_excluded_data);

/* Insert Join */
INSERT INTO Donation (year, amount, branch_id, donar_id)
SELECT 
    t."Financial Year",
    t."Amount",
    b.id AS branch_id,
    d.id AS donar_id
FROM temp_receipt_data t
JOIN Branch b ON b.name = t."Recipient Name"
JOIN Donar d ON d.name = t."Received From";
