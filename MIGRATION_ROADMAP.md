# Database Schema Migration Roadmap
## From Many-to-Many to One-to-Many Patient-User Relationship

**Created:** 2025-11-22
**Status:** Planning Phase

---

## Executive Summary

This document outlines the strategy to transform the current many-to-many relationship between users and patients into a one-to-many relationship (user has many patients, patient belongs to one user), while repurposing the `patient_users` table into a `medical_appointments` table.

### Key Changes
- Add `user_id` foreign key to `patients` table
- Rename `patient_users` table to `medical_appointments`
- Change primary key from composite `(user_id, patient_id)` to auto-increment `id`
- Migrate all existing patient-user links to appointments
- Update all application code to use new schema

---

## Current State Analysis

### Database Schema

#### `patients` Table
```
- id (PK, serial)
- created_at, updated_at
- pid (UUID, unique)
- ssn (encrypted)
- hashed_ssn
- first_name, last_name
- address_line_1, address_zip_code, address_city, address_country
- email
```

#### `patient_users` Table (Join Table)
```
- user_id (PK, FK -> users.id)
- patient_id (PK, FK -> patients.id)
- practitioner_office_id (FK -> practitioner_offices.id)
- created_at, updated_at
```

### Current Relationships
- **Users ↔ Patients**: Many-to-Many via `patient_users`
- **Patient_Users → Practitioner_Offices**: Many-to-One

### Usage Patterns
1. **Creation**: `services/patients.rs:15-46` - Creates patient and links to user
2. **Update**: `services/patients.rs:48-72` - Updates patient and practitioner office
3. **Search**: `services/patients.rs:74-119` - Queries patients by user with office info
4. **Authorization**: `controllers/patient.rs:55-60` - Checks user owns patient

---

## Target State

### Database Schema

#### `patients` Table (Modified)
```
- id (PK, serial)
- user_id (FK -> users.id, NOT NULL) ← NEW
- created_at, updated_at
- pid (UUID, unique)
- ssn (encrypted)
- hashed_ssn
- first_name, last_name
- address_line_1, address_zip_code, address_city, address_country
- email
```

#### `medical_appointments` Table (New Table - replaces patient_users)
```
- id (PK, serial, auto_increment) ← NEW
- user_id (FK -> users.id, NOT NULL)
- patient_id (FK -> patients.id, NOT NULL)
- practitioner_office_id (FK -> practitioner_offices.id, NOT NULL)
- created_at, updated_at
```

**Migration Note:** This table will be created fresh with the correct structure, then data will be copied from `patient_users`, and finally `patient_users` will be dropped. This approach is safer than renaming and modifying in place.

### New Relationships
- **Users → Patients**: One-to-Many (user has many patients)
- **Medical_Appointments → Users**: Many-to-One
- **Medical_Appointments → Patients**: Many-to-One
- **Medical_Appointments → Practitioner_Offices**: Many-to-One

---

## Migration Strategy

### Phase 1: Preparation & Assessment
**Estimated Duration:** 1-2 hours

1. **Backup Database**
   - Create full database backup
   - Document current row counts for verification
   - Test backup restoration procedure

2. **Data Validation**
   - Verify no orphaned records in `patient_users`
   - Check for data integrity issues
   - Identify any patients with multiple users (will need special handling)
   - Document edge cases

3. **Code Audit**
   - List all files referencing `patient_users` (see References section below)
   - Identify all database queries to be updated
   - Map entity relationships to be modified

### Phase 2: Schema Migration
**Estimated Duration:** 2-3 hours

1. **Create Migration File**
   - Create new migration: `m20251122_XXXXXX_restructure_patient_user_relationship.rs`

2. **Migration Steps (UP)**
   ```sql
   -- Step 1: Add user_id to patients table (nullable initially)
   ALTER TABLE patients ADD COLUMN user_id INTEGER REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE;

   -- Step 2: Populate user_id in patients from patient_users
   -- For patients with multiple users, choose the earliest created_at (first user who added them)
   UPDATE patients p
   SET user_id = (
     SELECT pu.user_id
     FROM patient_users pu
     WHERE pu.patient_id = p.id
     ORDER BY pu.created_at ASC
     LIMIT 1
   );

   -- Step 3: Make user_id NOT NULL
   ALTER TABLE patients ALTER COLUMN user_id SET NOT NULL;

   -- Step 4: Create index on user_id for performance
   CREATE INDEX idx_patients_user_id ON patients(user_id);

   -- Step 5: Create new medical_appointments table with correct structure
   CREATE TABLE medical_appointments (
     id SERIAL PRIMARY KEY,
     user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
     patient_id INTEGER NOT NULL REFERENCES patients(id) ON DELETE CASCADE ON UPDATE CASCADE,
     practitioner_office_id INTEGER NOT NULL REFERENCES practitioner_offices(id) ON DELETE CASCADE ON UPDATE CASCADE,
     date DATE NOT NULL,
     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
   );

   -- Step 6: Copy all data from patient_users to medical_appointments
   INSERT INTO medical_appointments (user_id, patient_id, practitioner_office_id, created_at, updated_at)
   SELECT user_id, patient_id, practitioner_office_id, created_at, updated_at
   FROM patient_users;

   -- Step 7: Create indexes on foreign keys for performance
   CREATE INDEX idx_medical_appointments_user_id ON medical_appointments(user_id);
   CREATE INDEX idx_medical_appointments_patient_id ON medical_appointments(patient_id);
   CREATE INDEX idx_medical_appointments_office_id ON medical_appointments(practitioner_office_id);

   -- Step 8: CRITICAL - VALIDATION CHECKPOINT
   -- ⚠️  STOP HERE and run the "Pre-Drop Validation" queries (see that section)
   -- ⚠️  Only proceed to Step 9 if ALL validation queries pass!
   -- This is a manual checkpoint - do not automate past this point without verification

   -- Step 9: Drop the old patient_users table (only after validation passes!)
   DROP TABLE patient_users;
   ```

3. **Migration Steps (DOWN)**
   ```sql
   -- Reverse the changes (for rollback capability)

   -- Step 1: Recreate patient_users table with original structure
   CREATE TABLE patient_users (
     user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
     patient_id INTEGER NOT NULL REFERENCES patients(id) ON DELETE CASCADE ON UPDATE CASCADE,
     practitioner_office_id INTEGER NOT NULL REFERENCES practitioner_offices(id) ON DELETE CASCADE ON UPDATE CASCADE,
     created_at TIMESTAMPTZ NOT NULL,
     updated_at TIMESTAMPTZ NOT NULL,
     PRIMARY KEY (user_id, patient_id)
   );

   -- Step 2: Copy data back from medical_appointments to patient_users
   INSERT INTO patient_users (user_id, patient_id, practitioner_office_id, created_at, updated_at)
   SELECT user_id, patient_id, practitioner_office_id, created_at, updated_at
   FROM medical_appointments;

   -- Step 3: Drop medical_appointments table
   DROP TABLE medical_appointments;

   -- Step 4: Remove user_id from patients table
   DROP INDEX IF EXISTS idx_patients_user_id;
   ALTER TABLE patients DROP COLUMN user_id;
   ```

### Phase 3: Code Updates
**Estimated Duration:** 4-6 hours

#### 3.1 Update Entity Definitions

**File:** `src/models/_entities/patients.rs`
- Add `user_id` field to `Model` struct
- Update `Relation` enum to include `Users` relation
- Add `Related<super::users::Entity>` implementation
- Update relation with `patient_users` to `medical_appointments`

**File:** `src/models/_entities/patient_users.rs` → Rename to `medical_appointments.rs`
- Rename struct from `patient_users` to `medical_appointments`
- Change primary key from composite to single `id` field
- Keep all foreign key relations

**Files to Update:**
- `src/models/_entities/users.rs` - Update relation to `medical_appointments`
- `src/models/_entities/practitioner_offices.rs` - Update relation to `medical_appointments`
- `src/models/_entities/mod.rs` - Rename export
- `src/models/_entities/prelude.rs` - Update prelude exports

#### 3.2 Update Model Logic

**File:** `src/models/patient_users.rs` → Rename to `medical_appointments.rs`
```rust
// Update to:
pub use super::_entities::medical_appointments::{ActiveModel, Entity, Model};
pub type MedicalAppointments = Entity;

pub struct CreateAppointmentParams {
  pub user_id: i32,
  pub patient_id: i32,
  pub practitioner_office_id: i32,
}

pub struct UpdateAppointmentParams {
  pub practitioner_office_id: i32,
}

impl ActiveModel {
  pub async fn create<T: ConnectionTrait>(
    db: &T,
    params: &CreateAppointmentParams,
  ) -> ModelResult<Model, MyErrors> {
    // No longer need to use composite key
    // Let id auto-increment
  }

  pub async fn update<T: ConnectionTrait>(
    db: &T,
    appointment_id: i32,  // Changed from user_id, patient_id
    params: &UpdateAppointmentParams,
  ) -> ModelResult<Model, MyErrors> {
    // Use single id instead of composite key
  }
}
```

**File:** `src/models/patients.rs`
- Update `CreatePatientParams` to include `user_id`
- Update `ActiveModel::create` to set `user_id` directly on patient
- Remove transaction complexity that creates patient_users link

#### 3.3 Update Services

**File:** `src/services/patients.rs`

**Before:**
```rust
pub async fn create(
  patient_params: &CreatePatientParams,
  linked_to_user: &users::Model,
) -> Result<PatientModel, MyErrors> {
  let db_transaction = services.db.begin().await?;
  let created_patient = patients::ActiveModel::create(&db_transaction, patient_params).await?;
  patient_users::ActiveModel::create(&db_transaction, &CreateLinkParams {...}).await?;
  db_transaction.commit().await?;
  Ok(created_patient)
}
```

**After:**
```rust
pub async fn create(
  patient_params: &CreatePatientParams,
  linked_to_user: &users::Model,
) -> Result<PatientModel, MyErrors> {
  let services = get_services();
  let db_transaction = services.db.begin().await?;

  // Create patient with user_id directly
  let created_patient = patients::ActiveModel::create(
    &db_transaction,
    patient_params,
    linked_to_user.id  // Pass user_id
  ).await?;

  // Create appointment record
  medical_appointments::ActiveModel::create(
    &db_transaction,
    &CreateAppointmentParams {
      user_id: linked_to_user.id,
      patient_id: created_patient.id,
      practitioner_office_id: patient_params.practitioner_office_id,
    },
  ).await?;

  db_transaction.commit().await?;
  Ok(created_patient)
}
```

**Update search_paginated:**
```rust
// Before: Inner join with patient_users to filter by user
let paginator = patients::Entity::find()
  .inner_join(patient_users::Entity)
  .filter(patient_users::Column::UserId.eq(user.id))

// After: Direct filter on patients.user_id
let paginator = patients::Entity::find()
  .filter(patients::Column::UserId.eq(user.id))
```

**Update update function:**
- Simplify to just update patient (no more patient_users update for practitioner_office)
- Create new appointment record if practitioner office changes

#### 3.4 Update Controllers

**File:** `src/controllers/patient.rs`

**Update authorization check:**
```rust
// Before: Check via join with patient_users
let patient = patients::Entity::find_by_id(patient_id)
  .inner_join(patient_users::Entity)
  .filter(patient_users::Column::UserId.eq(ctx.current_user().0.id))
  .one(&ctx.db)
  .await?
  .ok_or(ApplicationError::NOT_FOUND())?;

// After: Direct check on patient.user_id
let patient = patients::Entity::find_by_id(patient_id)
  .filter(patients::Column::UserId.eq(ctx.current_user().0.id))
  .one(&ctx.db)
  .await?
  .ok_or(ApplicationError::NOT_FOUND())?;
```

#### 3.5 Update Imports

**Files requiring import updates:**
- `src/models/mod.rs`
- `src/services/invoice.rs`
- Any test files

### Phase 4: Testing
**Estimated Duration:** 3-4 hours

1. **Unit Tests**
   - Test patient creation with user_id
   - Test appointment creation
   - Test patient search by user
   - Test authorization checks

2. **Integration Tests**
   - Test full patient workflow (create, update, search)
   - Test edge cases (patient with no appointments, multiple appointments)
   - Test data consistency after migration

3. **Manual Testing**
   - Create new patient via API
   - Update patient and verify appointment creation
   - Search patients
   - Generate invoice (verify practitioner_office resolution)

4. **Migration Testing**
   - Test migration UP on copy of production data
   - Verify row counts match expectations
   - Check for data loss
   - Test migration DOWN (rollback)
   - Re-test migration UP

### Phase 5: Deployment
**Estimated Duration:** 1-2 hours

1. **Pre-Deployment**
   - Announce maintenance window
   - Create final database backup
   - Prepare rollback plan

2. **Deployment Steps**
   - Put application in maintenance mode
   - Run database migration
   - Deploy new code
   - Run post-migration verification queries
   - Remove maintenance mode

3. **Post-Deployment**
   - Monitor error logs
   - Verify key workflows
   - Keep rollback plan ready for 24 hours

---

## Code References

### Files Requiring Updates

#### Database/Migration Files
1. `migration/src/m20250820_152922_create_join_table_users_and_patients.rs` - Reference only
2. **NEW:** `migration/src/m20251122_XXXXXX_restructure_patient_user_relationship.rs`

#### Model Files (Entity Definitions)
3. `src/models/_entities/patient_users.rs` → Rename to `medical_appointments.rs`
4. `src/models/_entities/patients.rs` - Add user_id field
5. `src/models/_entities/users.rs` - Update relation name
6. `src/models/_entities/practitioner_offices.rs` - Update relation name
7. `src/models/_entities/mod.rs` - Update exports
8. `src/models/_entities/prelude.rs` - Update exports

#### Model Files (Business Logic)
9. `src/models/patient_users.rs` → Rename to `medical_appointments.rs`
10. `src/models/patients.rs` - Update create/update logic
11. `src/models/mod.rs` - Update module exports

#### Service Files
12. `src/services/patients.rs` - Major refactoring
13. `src/services/invoice.rs` - Update to use medical_appointments

#### Controller Files
14. `src/controllers/patient.rs` - Update authorization logic

#### View Files
15. `src/views/patient.rs` - Potentially update response structure

---

## Data Integrity Considerations

### Patients with Multiple Users
**Issue:** In the current many-to-many model, a patient could belong to multiple users. When migrating to one-to-many, we must choose one user per patient.

**Solution:**
- Migration chooses the user who first created the link (earliest `created_at`)
- All existing `patient_users` records migrate to `medical_appointments`
- This preserves the history: patient belongs to User A, but has appointments with both User A and User B

**Example:**
```
Before:
Patient 123 ↔ User 1 (created_at: 2024-01-01)
Patient 123 ↔ User 2 (created_at: 2024-02-01)

After:
Patient 123.user_id = 1 (owns the patient)
Medical_Appointment: {patient_id: 123, user_id: 1, ...}
Medical_Appointment: {patient_id: 123, user_id: 2, ...} (historical appointment)
```

### Orphaned Records
- Migration includes CASCADE deletes to prevent orphans
- Pre-migration validation identifies existing orphans

---

## Risks and Mitigation

**Migration Strategy:** We use a **create-copy-drop** approach instead of in-place table modification:
- ✅ Original `patient_users` data remains intact during migration
- ✅ Easier validation (compare both tables before dropping old one)
- ✅ Simpler rollback (just drop new table)
- ✅ No complex ALTER operations on production data
- ✅ Can pause migration at any step to verify

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Data loss during migration | HIGH | LOW | Full backup + Create-copy-drop strategy preserves original data until final step |
| Patients with multiple users lose access | MEDIUM | MEDIUM | Document affected patients, provide admin tool to reassign |
| Application downtime | MEDIUM | LOW | Maintenance window, rollback plan ready |
| Performance degradation | LOW | LOW | Added indexes on new columns |
| Breaking existing integrations | HIGH | MEDIUM | Comprehensive testing, version API if needed |
| Foreign key constraint violations | MEDIUM | LOW | Copy preserves all FKs, validation step before dropping old table |

---

## Rollback Plan

### Scenario 1: Migration Fails (Before Code Deploy)
**Advantage:** With create-copy-drop strategy, original `patient_users` table may still exist if failure happened before final DROP.

1. If `patient_users` still exists:
   - Simply drop `medical_appointments` table (if created)
   - Remove `user_id` from `patients` table (if added)
   - No data restoration needed!
2. If `patient_users` was already dropped:
   - Run migration DOWN (recreates patient_users from medical_appointments)
   - Or restore from backup if migration DOWN fails
3. Investigate and fix migration script
4. Retry

### Scenario 2: Post-Deployment Issues
1. Enable maintenance mode
2. Deploy previous code version
3. Run migration DOWN
4. Restore database from backup
5. Verify system functionality
6. Investigate root cause

### Scenario 3: Data Inconsistency Discovered
1. Document inconsistencies
2. If minor: Create data fix script
3. If major: Full rollback per Scenario 2
4. Fix and re-test migration

---

## Pre-Drop Validation (During Migration)

**IMPORTANT:** Run these queries BEFORE executing the final `DROP TABLE patient_users` step!

```sql
-- 1. Verify row counts match exactly
SELECT
  (SELECT COUNT(*) FROM patient_users) as original_count,
  (SELECT COUNT(*) FROM medical_appointments) as new_count,
  (SELECT COUNT(*) FROM patient_users) = (SELECT COUNT(*) FROM medical_appointments) as counts_match;
-- Expected: counts_match = true

-- 2. Verify all data was copied correctly (random sample check)
SELECT pu.user_id, pu.patient_id, pu.practitioner_office_id, pu.created_at,
       ma.user_id as ma_user_id, ma.patient_id as ma_patient_id,
       ma.practitioner_office_id as ma_office_id, ma.created_at as ma_created_at
FROM patient_users pu
LEFT JOIN medical_appointments ma
  ON pu.user_id = ma.user_id
  AND pu.patient_id = ma.patient_id
  AND pu.practitioner_office_id = ma.practitioner_office_id
LIMIT 10;
-- Verify: All ma_* columns should have matching values

-- 3. Verify no records were missed
SELECT COUNT(*) FROM patient_users pu
WHERE NOT EXISTS (
  SELECT 1 FROM medical_appointments ma
  WHERE ma.user_id = pu.user_id
    AND ma.patient_id = pu.patient_id
    AND ma.practitioner_office_id = pu.practitioner_office_id
);
-- Expected: 0

-- 4. Check for any NULL values that shouldn't exist
SELECT COUNT(*) FROM medical_appointments
WHERE user_id IS NULL OR patient_id IS NULL OR practitioner_office_id IS NULL;
-- Expected: 0
```

**Only proceed with `DROP TABLE patient_users` if all validation queries pass!**

---

## Post-Migration Validation

### SQL Verification Queries

```sql
-- 1. Verify all patients have a user_id
SELECT COUNT(*) FROM patients WHERE user_id IS NULL;
-- Expected: 0

-- 2. Verify medical_appointments table exists and is accessible
SELECT COUNT(*) FROM medical_appointments;
-- Expected: Should return total count without errors

-- 3. Verify no orphaned appointments
SELECT COUNT(*)
FROM medical_appointments ma
WHERE NOT EXISTS (SELECT 1 FROM patients p WHERE p.id = ma.patient_id);
-- Expected: 0

-- 4. Verify patient ownership distribution
SELECT user_id, COUNT(*) as patient_count
FROM patients
GROUP BY user_id
ORDER BY patient_count DESC;

-- 5. Verify appointments per patient
SELECT patient_id, COUNT(*) as appointment_count
FROM medical_appointments
GROUP BY patient_id
HAVING COUNT(*) > 1;
```

### Application Verification
- [ ] Create new patient → Success
- [ ] Update patient → Success
- [ ] Search patients → Returns correct results
- [ ] Generate invoice → Includes correct practitioner office
- [ ] Patient list pagination → Works correctly
- [ ] Authorization checks → Prevent unauthorized access

---

## Timeline Summary

| Phase | Duration | Can Start |
|-------|----------|-----------|
| 1. Preparation | 1-2 hours | Immediately |
| 2. Schema Migration Development | 2-3 hours | After Phase 1 |
| 3. Code Updates | 4-6 hours | During Phase 2 |
| 4. Testing | 3-4 hours | After Phase 3 |
| 5. Deployment | 1-2 hours | After Phase 4 |
| **Total** | **11-17 hours** | |

---

## Success Criteria

- [ ] All patients have exactly one owner (user_id)
- [ ] All existing patient-user relationships preserved as medical appointments
- [ ] No data loss (row counts match)
- [ ] All tests pass
- [ ] Application functions correctly with new schema
- [ ] No performance degradation
- [ ] Rollback tested and ready
- [ ] Documentation updated

---

## Future Considerations

### Potential Enhancements
1. **Appointment Date/Time**: Add when appointment scheduling is needed
2. **Appointment Status**: Track scheduled/completed/cancelled appointments
3. **Patient Transfer**: Add functionality to transfer patient ownership between users
4. **Appointment Notes**: Allow practitioners to add notes to appointments
5. **Soft Deletes**: Consider soft deletes for appointments (audit trail)

### Schema Evolution
- This migration makes the schema more normalized and scalable
- Future appointment features can be added without affecting patient ownership
- Consider adding a `patient_history` table to track ownership changes

---

## Questions and Decisions Log

**Q1:** What should happen to patients with multiple users during migration?
**A:** Patient is assigned to the user who created the link first (earliest created_at). All links become medical appointments.

**Q2:** Should we add appointment date/time now?
**A:** No, use created_at for now. Can add later if needed.

**Q3:** What should the new table be called?
**A:** `medical_appointments`

**Q4:** What to do with existing patient_users records?
**A:** Migrate all to medical_appointments table.

---

## Additional Notes

- Generated entities in `src/models/_entities/` are auto-generated by SeaORM
- After migration, run entity generation to update these files: `sea-orm-cli generate entity`
- Consider adding migration to seed data for testing
- Update any API documentation to reflect new structure
- Coordinate with frontend team if changes affect API responses

---

**Document Version:** 1.0
**Last Updated:** 2025-11-22
**Author:** Migration Planning Team
