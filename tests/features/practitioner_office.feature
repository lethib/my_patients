Feature: Practitioner office management
  As a practitioner
  I want to manage my offices
  In order to track where I practice

  Background:
    Given a practitioner exists for office tests

  Rule: Offices can be created and linked to a practitioner

    Scenario: Create an office linked to a practitioner with a revenue share
      When I create an office "Cabinet du Sud" with a revenue share of 30
      Then the office is linked to the practitioner with a revenue share of 30

    Scenario: Cannot create an office with an invalid zip code
      When I try to create an office with an invalid zip code
      Then the office creation fails

  Rule: Offices can be updated

    Scenario: Update an office's revenue share percentage
      Given an office "Cabinet du Nord" linked to the practitioner with a revenue share of 20
      When I update the office revenue share to 45
      Then the office revenue share is 45

    Scenario: Update an office name
      Given an office "Old Name" linked to the practitioner with a revenue share of 25
      When I update the office name to "New Name"
      Then the office name is "New Name"

    Scenario: Update an office trims whitespace
      Given an office "My Office" linked to the practitioner with a revenue share of 10
      When I update the office name to "  Trimmed Name  "
      Then the office name is "Trimmed Name"
