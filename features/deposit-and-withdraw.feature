Feature: A simple toy payments engine deposits and withdraws
  Scenario: Simple deposits and withdraws
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    deposit,    2,        2,    3.0
    deposit,    1,        3,    2.0
    withdrawal, 1,        4,    1.5
    withdrawal, 2,        5,    1.0
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,1.5,0.0,1.5,false
    2,2.0,0.0,2.0,false
    """

  Scenario: Withdraws cannot go under zero
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    withdrawal, 1,        2,    1.5
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,1.0,0.0,1.0,false
    """

  Scenario: Withdraw the full amount
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    withdrawal, 1,        2,    1.0
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,0.0,0.0,0.0,false
    """