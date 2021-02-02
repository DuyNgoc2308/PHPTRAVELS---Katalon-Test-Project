#Author: your.email@your.domain.com
#Keywords Summary :
#Feature: List of scenarios.
#Scenario: Business rule through list of steps with arguments.
#Given: Some precondition step
#When: Some key actions
#Then: To observe outcomes or validation
#And,But: To enumerate more Given,When,Then steps
#Scenario Outline: List of steps for data-driven as an Examples and <placeholder>
#Examples: Container for s table
#Background: List of steps run before each of the scenarios
#""" (Doc Strings)
#| (Data Tables)
#@ (Tags/Labels):To group Scenarios
#<> (placeholder)
#""
## (Comments)
#Sample Feature Definition Template'
#Sign Up feature definition
@Sign_Up
Feature: Sign up feature

@Valid
  Scenario Outline: Sign up with valid credentials
    Given I navigate to Sign Up page
    When I enter <firstname>, <lastname>, <phone>
    And I type <email>, <password>
    And I click on Sign Up button
    Then I should be able to Sign Up sucessfully

    Examples: 
      | firstname | lastname  | phone   | email           | password |
      | john      | willinton | 1234567 | johnw@gmail.com | abc1234  |
