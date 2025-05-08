DROP TABLE IF EXISTS Donor CASCADE;
DROP TABLE IF EXISTS Donation CASCADE;
DROP TABLE IF EXISTS Party CASCADE;
DROP TABLE IF EXISTS Branch CASCADE;

-- Table structure definitions
CREATE TABLE Branch
(
  id       serial  NOT NULL,
  name     varchar NOT NULL,
  party_id integer NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE Donor
(
  id          serial  NOT NULL,
  name        varchar NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE Donation
(
  id        serial  NOT NULL,
  year      varchar NOT NULL,
  amount    bigint  NOT NULL,
  branch_id integer  NOT NULL,
  donor_id  integer  NOT NULL,
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

ALTER TABLE Donation
  ADD CONSTRAINT FK_Branch_TO_Donation
    FOREIGN KEY (branch_id)
    REFERENCES Branch (id)
    ON DELETE CASCADE;

ALTER TABLE Donation
  ADD CONSTRAINT FK_Donor_TO_Donation
    FOREIGN KEY (donor_id)
    REFERENCES Donor (id)
    ON DELETE CASCADE;

ALTER TABLE Party
  ADD CONSTRAINT unique_party_name UNIQUE (name);

ALTER TABLE Branch
  ADD CONSTRAINT unique_branch_name UNIQUE (name);

ALTER TABLE Donor
  ADD CONSTRAINT unique_donor_name UNIQUE (name);

-- Prepopulate and prune based on receipts
CREATE TEMP TABLE TempReceipt
(
    "Financial Year" TEXT NOT NULL,
    "Return Type" TEXT NOT NULL,
    "Recipient Name" TEXT NOT NULL,
    "Received From" TEXT NOT NULL,
    "Receipt Type" TEXT,
    "Amount" BIGINT NOT NULL
);
\copy TempReceipt FROM 'DetailedReceipts.csv' DELIMITER ',' CSV HEADER;
DELETE FROM TempReceipt WHERE "Return Type" NOT IN ('Political Party Return', 'Member of HOR Return');

-- Used defined JSON to get the list of parties and branches
CREATE TEMP TABLE RawParties (data jsonb);
\copy RawParties FROM 'Parties.json';

DO $$
DECLARE
    party_name text;
    children jsonb;
    party_id integer;
    branch_name text;
BEGIN
    FOR party_name, children IN
        SELECT key, value
        FROM jsonb_each((SELECT data FROM RawParties LIMIT 1))
    LOOP
        -- Insert party
        INSERT INTO Party (name) VALUES (party_name) RETURNING id INTO party_id;

        -- If value is a string (e.g. "null"), just insert the party name as a branch
        IF jsonb_typeof(children) = 'string' THEN
            INSERT INTO Branch (name, party_id) VALUES (party_name, party_id);

        -- If value is an array, insert each child as a branch
        ELSIF jsonb_typeof(children) = 'array' THEN
            FOR branch_name IN
                SELECT jsonb_array_elements_text(children)
            LOOP
                INSERT INTO Branch (name, party_id) VALUES (branch_name, party_id);
            END LOOP;

        ELSE
            RAISE NOTICE 'Unexpected value type for %: %', party_name, children;
        END IF;
    END LOOP;
END $$;

-- Fill the donors now
INSERT INTO Donor (name)
SELECT DISTINCT "Received From" FROM TempReceipt;

-- Finally insert the 1-many table
INSERT INTO Donation (year, amount, branch_id, donor_id)
SELECT
    t."Financial Year",
    t."Amount",
    b.id AS branch_id,
    d.id AS donor_id
FROM TempReceipt t
JOIN Branch b ON b.name = t."Recipient Name"
JOIN Donor d ON d.name = t."Received From";

-- Drop temp tables
DROP TABLE TempReceipt CASCADE;
DROP TABLE RawParties CASCADE;
