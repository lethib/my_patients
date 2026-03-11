Feature: Medical appointment management
  As a practitioner
  I want to manage medical appointments
  In order to track my activity

  Background:
    Given a practitioner exists
    And a practitioner office "Cabinet Central" exists with revenue share 70
    And a patient "Alice" "Dupont" exists

  Rule: Appointments can be created and updated

    Scenario: Create an appointment
      When I create an appointment on "2026-03-15" at price 5000
      Then the appointment is saved with date "2026-03-15"

    Scenario: Create an appointment with a payment method
      When I create an appointment on "2026-03-15" at price 5000 with payment "cash"
      Then the appointment payment method is "cash"

    Scenario: Update an appointment date
      Given an appointment on "2026-03-15" at price 5000
      When I update the appointment date to "2026-04-20"
      Then the appointment date is "2026-04-20"

  Rule: Appointments can be extracted by date range

    Scenario: Extract appointments within a date range
      Given an appointment on "2026-03-10" at price 3000
      And an appointment on "2026-03-20" at price 4500
      When I extract appointments between "2026-03-01" and "2026-03-31"
      Then 2 appointments are returned

    Scenario: No appointments outside the date range are returned
      Given an appointment on "2026-02-15" at price 3000
      When I extract appointments between "2026-03-01" and "2026-03-31"
      Then 0 appointments are returned

  Rule: Extracted appointments include the revenue share percentage

    Scenario: Revenue share percentage is included in extracted appointments
      Given an appointment on "2026-03-10" at price 10000
      When I extract appointments between "2026-03-01" and "2026-03-31"
      Then the first extracted appointment has a revenue share of 70.0

    Scenario: Each office has its own revenue share percentage
      Given a second office "Cabinet Sud" exists with revenue share 50
      And an appointment on "2026-03-10" at price 10000
      And an appointment on "2026-03-15" at price 8000 at office "Cabinet Sud"
      When I extract appointments between "2026-03-01" and "2026-03-31"
      Then 2 appointments are returned
      And the extracted appointment for office "Cabinet Central" has a revenue share of 70.0
      And the extracted appointment for office "Cabinet Sud" has a revenue share of 50.0
