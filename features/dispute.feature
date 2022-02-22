Feature: A simple toy payments engine dispute a transaction
  Scenario: Dispute a transaction
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    3.0
    deposit,    1,        2,    1.0
    dispute,    1,        2,
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,3.0,1.0,4.0,false
    """

  Scenario: Disputes affect withdraws
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    dispute,    1,        1,
    withdrawal, 1,        2,    0.5
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,0.0,1.0,1.0,false
    """
