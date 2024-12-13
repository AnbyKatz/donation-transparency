DROP TABLE Donar CASCADE;
DROP TABLE Donations CASCADE;
DROP TABLE Party CASCADE;

/* auto generated here */
CREATE TABLE Donar
(
  id       serial  NOT NULL,
  name     varchar NOT NULL,
  industry varchar,
  parent   varchar,
  PRIMARY KEY (id)
);

CREATE TABLE Donations
(
  id       serial  NOT NULL,
  year     varchar NOT NULL,
  amount   bigint  NOT NULL,
  party_id serial  NOT NULL,
  donar_id serial  NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE Party
(
  id     serial  NOT NULL,
  name   varchar NOT NULL,
  parent varchar,
  PRIMARY KEY (id)
);

ALTER TABLE Donations
  ADD CONSTRAINT FK_Party_TO_Donations
    FOREIGN KEY (party_id)
    REFERENCES Party (id);

ALTER TABLE Donations
  ADD CONSTRAINT FK_Donar_TO_Donations
    FOREIGN KEY (donar_id)
    REFERENCES Donar (id);

/* ends here */

/* extra constraints */
ALTER TABLE Party
  ADD CONSTRAINT unique_party_name UNIQUE (name);

ALTER TABLE Donar
  ADD CONSTRAINT unique_donar_name UNIQUE (name);  

/* CSV inserts */
DROP table temp_data;
CREATE TEMP TABLE temp_data (
    "Financial Year" TEXT,
    "Return Type" TEXT,
    "Recipient Name" TEXT,
    "Received From" TEXT,
    "Receipt Type" TEXT,
    "Amount" BIGINT
);

\copy temp_data
FROM 'DetailedReceipts.csv'
DELIMITER ','
CSV HEADER;

/* Insert to get IDs */
INSERT INTO Party (name) 
  SELECT DISTINCT "Recipient Name" FROM temp_data;
INSERT INTO Donar (name) 
  SELECT DISTINCT "Received From" FROM temp_data;

/* Insert Join */
INSERT INTO Donations (year, amount, party_id, donar_id)
SELECT 
    t."Financial Year",
    t."Amount",
    p.id AS party_id,
    d.id AS donar_id
FROM temp_data t
JOIN Party p ON p.name = t."Recipient Name"
JOIN Donar d ON d.name = t."Received From";


DROP table temp_data;
