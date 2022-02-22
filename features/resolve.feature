Feature: A simple toy payments engine resolve a disputed transaction
  Scenario: Resolve a previously disputed transaction
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    dispute,    1,        1,
    resolve,    1,        1,
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,1.0,0.0,1.0,false
    """

  Scenario: Resolves affect withdraws
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    dispute,    1,        1,
    resolve,    1,        1,
    withdrawal, 1,        2,    0.5
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,0.5,0.0,0.5,false
    """

  Scenario: Cannot resolve a not disputed transaction
    Given the following CSV file
    """
    type,       client,   tx,   amount
    deposit,    1,        1,    1.0
    resolve,    1,        1,
    withdrawal, 1,        2,    0.5
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,0.5,0.0,0.5,false
    """
