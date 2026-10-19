Supported Statements
====================

The Transtractor uses rules-based parsing to extract transaction data from bank statements. Each 
supported statement format is defined by a specific set of parsing rules tailored to the bank 
and account type. The tables below summarise the supported statements.

Statement is not Listed
------------------------
You can also create your own configuration file following the guidelines in the 
`Creating Configuration Files <configuration.html>`_ section. 

Otherwise, you can send a redacted `layout file` to developer@transtractor.net. Layout 
files can be generated using Python:


.. code-block:: python

    from transtractor import Parser
    parser = Parser()
    parser.layout("path/to/your/statement.pdf", "path/to/write/your/layout.txt")

Or using the `add support for your statements page <https://www.transtractor.net/add-support-for-your-statements>`_
on the website. See this webpage for instructions explaining how these files are redacted.

This provides me the information required for hard-coding parsing rules for your statements.

Supported Statements by Country
-------------------------------

Statements marked as `Unreleased` can be processed on the `website <https://www.transtractor.net/>`_ or 
using the source code directly. These will be available in future PyPI releases.

Statements marked with an asterisk (\*) indicate that they bundle transactions 
from multiple account types into a single PDF. The Transtractor extracts them 
individually and assigns each transaction to the correct account based on the 
account numbers.

Australia
***********

.. list-table::
    :header-rows: 1
    :widths: 40 30 15 15

    * - Bank
      - Example Accounts
      - Introduced
      - Key
    * - Commonwealth Bank
      - Low Rate MasterCard, Low Fee Mastercard
      - v0.9.0
      - ``au__cba__credit_card__1``
    * - Commonwealth Bank
      - Streamline, Smart Access, GoalSaver, Everyday Offset
      - v0.9.0
      - ``au__cba__debit__1``
    * - Commonwealth Bank
      - Complete Home Loan
      - v0.9.0
      - ``au__cba__loan__1``
    * - Defence Bank
      - Foundation Credit Card, Low Start Credit Card
      - Unreleased
      - ``au__def__credit_card__1``
    * - Defence Bank
      - iSaver Intro\*
      - Unreleased
      - ``au__def__isaver_intro__1``
    * - Defence Bank
      - Max eSaver\*
      - Unreleased
      - ``au__def__max_esaver__1``
    * - Defence Bank
      - Personal Loan
      - Unreleased
      - ``au__def__personal_loan__1``
    * - ING Bank
      - Orange Everyday, Savings Maximiser
      - v0.12.0
      - ``au__ing__debit__1``
    * - National Australia Bank
      - Classic Banking
      - v0.9.0
      - ``au__nab__classic_banking__1``
    * - Westpac Banking Corporation
      - Choice, Life
      - v0.12.0
      - ``au__wbc__debit__1``

Thailand
***********

.. list-table::
    :header-rows: 1
    :widths: 40 30 15 15

    * - Bank
      - Example Accounts
      - Introduced
      - Key
    * - Bangkok Bank
      - Savings Account
      - v0.15.0
      - ``th__bbl__savings__1``

United States
**************

.. list-table::
    :header-rows: 1
    :widths: 40 30 15 15

    * - Bank
      - Example Accounts
      - Introduced
      - Key
    * - American Express
      - Platinum Card
      - v0.13.0
      - ``us__axp__platinum__1``
    * - Capital One
      - 360 Checking\*
      - v0.15.0
      - ``us__cof__360_checking__1``
    * - Capital One
      - 360 Performance Savings
      - v0.15.0
      - ``us__cof__360_performance_savings__1``
