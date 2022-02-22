Feature: A simple toy payments engine
  Scenario: Unknown transaction type is ignored
    Given the following CSV file
    """
    type,         client,   tx,   amount
    deposit,      1,        1,    1.0
    idkwhatitis
    deposit,      1,        3,    1.5
    """
    When the engine is executed
    Then the following output should be generated
    """
    client,available,held,total,locked
    1,2.5,0.0,2.5,false
    """
