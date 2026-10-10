Supported Statements
====================

The Transtractor uses rules-based parsing to extract transaction data from bank statements. Each 
supported statement format is defined by a specific set of parsing rules tailored to the bank 
and account type. These configuration are defined in Rust modules located in `src/configs/registry` of 
the `source code <https://github.com/weberdak/transtractor-lib>`_.

The following statements will be recognised and parsed automatically. You must create and load 
your own configuration files if your bank or account type is not listed here. Statements marked
`Unreleased` can be processed on the `website <https://www.transtractor.net/>`_ or using the source 
code directly but are not yet included in a PyPI release.

Australia
---------

.. list-table::
    :header-rows: 1
    :widths: 15 40 30 15

    * - Key
      - Bank
      - Example Accounts
      - Introduced
    * - ``au__cba__credit_card__1``
      - Commonwealth Bank
      - Low Rate MasterCard, Low Fee Mastercard
      - v0.9.0
    * - ``au__cba__debit__1``
      - Commonwealth Bank
      - Streamline, Smart Access, GoalSaver, Everyday Offset
      - v0.9.0
    * - ``au__cba__loan__1``
      - Commonwealth Bank
      - Complete Home Loan
      - v0.9.0
    * - ``au__nab__classic_banking__1``
      - National Australia Bank
      - Classic Banking
      - v0.9.0
    * - ``au__wbc__debit__1``
      - Westpac Banking Corporation
      - Choice, Life
      - v0.12.0
    * - ``au__ing__debit__1``
      - ING Bank
      - Orange Everyday, Savings Maximiser
      - v0.12.0

Thailand
--------

.. list-table::
    :header-rows: 1
    :widths: 15 40 30 15

    * - Key
      - Bank
      - Example Accounts
      - Introduced
    * - ``th__bbl__savings__1``
      - Bangkok Bank
      - Savings Account
      - Unreleased

United States
-------------

.. list-table::
    :header-rows: 1
    :widths: 15 40 30 15

    * - Key
      - Bank
      - Example Accounts
      - Introduced
    * - ``us__axp__platinum__1``
      - American Express
      - Platinum Card
      - v0.13.0
    * - ``us__cof__360_combo__1``\*
      - Capital One
      - 360 Checking, 360 Performance Savings
      - v0.14.0

\* Capital One 360 statements bundle transactions from multiple account types into a 
single PDF. The Transtractor merges these transactions into a single table and assigns 
them against the first account number found in the statement.