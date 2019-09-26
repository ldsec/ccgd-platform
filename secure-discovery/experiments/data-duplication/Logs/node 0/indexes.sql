-- Constraint: observation_fact_pk

-- ALTER TABLE i2b2demodata_i2b2.observation_fact DROP CONSTRAINT observation_fact_pk;

ALTER TABLE i2b2demodata_i2b2.observation_fact
    ADD CONSTRAINT observation_fact_pk PRIMARY KEY (patient_num, concept_cd, modifier_cd, start_date, encounter_num, instance_num, provider_id);

-- Index: of_idx_allobservation_fact

-- DROP INDEX i2b2demodata_i2b2.of_idx_allobservation_fact;

CREATE INDEX of_idx_allobservation_fact
    ON i2b2demodata_i2b2.observation_fact USING btree
    (patient_num, encounter_num, concept_cd COLLATE pg_catalog."default", start_date, provider_id COLLATE pg_catalog."default", modifier_cd COLLATE pg_catalog."default", instance_num, valtype_cd COLLATE pg_catalog."default", tval_char COLLATE pg_catalog."default", nval_num, valueflag_cd COLLATE pg_catalog."default", quantity_num, units_cd COLLATE pg_catalog."default", end_date, location_cd COLLATE pg_catalog."default", confidence_num)
    TABLESPACE pg_default;

-- Index: of_idx_clusteredconcept

-- DROP INDEX i2b2demodata_i2b2.of_idx_clusteredconcept;

CREATE INDEX of_idx_clusteredconcept
    ON i2b2demodata_i2b2.observation_fact USING btree
    (concept_cd COLLATE pg_catalog."default")
    TABLESPACE pg_default;

-- Index: of_idx_encounter_patient

-- DROP INDEX i2b2demodata_i2b2.of_idx_encounter_patient;

CREATE INDEX of_idx_encounter_patient
    ON i2b2demodata_i2b2.observation_fact USING btree
    (encounter_num, patient_num, instance_num)
    TABLESPACE pg_default;

