Feature: A simple toy payments engine chargeback a disputed transaction
  Scenario: Chargeback a previously disputed transaction
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    deposit,    1,        2,    1.0
    dispute,    1,        1,
    chargeback, 1,        1,
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,1.0,0.0,1.0,true
    """

  Scenario: Cannot withdraw from a locked account
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    deposit,    1,        2,    1.0
    dispute,    1,        1,
    chargeback, 1,        1,
    withdrawal, 1,        3,    0.5
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,1.0,0.0,1.0,true
    """

  Scenario: Cannot deposit to a locked account
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    deposit,    1,        2,    1.0
    dispute,    1,        1,
    chargeback, 1,        1,
    deposit,    1,        3,    0.5
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,1.0,0.0,1.0,true
    """
