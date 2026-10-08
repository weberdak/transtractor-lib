Creating Configuration Files
============================

Configuration files specify the parsing parameters that the Transtractor uses to extract 
data from bank statements. This guide explains how to create your own configuration files 
for unsupported banks or account types.

Online Tools
------------
You can use the `Transtractor online developer tools <https://www.transtractor.net/develop>`_ to develop 
a configuration file without installing Python. The page embeds the Transtractor as WebAssembly (WASM), 
so all processing takes place locally in your browser. This page is sufficient unless you need to modify 
the Transtractor source code.

If you prefer an easier process, follow Method 2 on the
`add support for your statements page <https://www.transtractor.net/add-support-for-your-statements>`_.
This provides a developer with the information needed to create your configuration and add permanent support
for reading your statements.



Basic Template
--------------
The JSON file used to read the
`test1.pdf <https://github.com/weberdak/transtractor-lib/blob/main/tests/fixtures/test1.pdf>`_ 
example file included in the source code is:

.. code-block:: json

    {
        "key": "au__gtb__fake_account__1",
        "bank_name": "Gravy Toast Bank",
        "account_type": "Savings",
        "account_terms": ["Gravy Toast", "Fake"],
        "account_examples": ["Fake Account Product", "Similar Product"],

        "account_number_terms": ["Account number:"],
        "account_number_patterns": ["\\b\\d{4}\\s\\d{4}\\s\\d{4}\\s\\d{4}\\b"],
        "account_number_alignment": "y1",
        "account_number_alignment_tol": 5,

        "opening_balance_terms": ["Opening balance:"],
        "opening_balance_formats": ["format3"],
        "opening_balance_alignment": "y1",
        "opening_balance_alignment_tol": 5,
        "opening_balance_invert": false,

        "closing_balance_terms": ["Closing balance:"],
        "closing_balance_formats": ["format3"],
        "closing_balance_alignment": "y1",
        "closing_balance_alignment_tol": 5,
        "closing_balance_invert": false,

        "start_date_terms": ["Statement Period:"],
        "start_date_formats": ["format2"],
        "start_date_alignment": "y1",
        "start_date_alignment_tol": 5,

        "transaction_terms": ["Transaction Details"],
        "transaction_terms_stop": ["Transactions stop here."],
        "transaction_formats": [
            ["date", "description", "amount", "balance"],
            ["date", "description", "amount"],
            ["description", "amount", "balance"],
            ["description", "amount"]
        ],
        "transaction_start_date_required": true,
        "transaction_alignment_tol": 10,

        "transaction_date_formats": ["format1"],
        "transaction_date_headers": ["Date"],
        "transaction_date_alignment": "x1",
        "transaction_date_x1_range": [0, 10000],
        "transaction_date_x2_range": [0, 10000],

        "transaction_description_headers": ["Description"],
        "transaction_description_alignment": "x1",
        "transaction_description_x1_range": [0, 10000],
        "transaction_description_x2_range": [0, 10000],
        "transaction_description_exclude": [
            " Annoying text",
            " to filter out"
        ],

        "transaction_amount_formats": ["format1", "format2"],
        "transaction_amount_headers": ["Credit"],
        "transaction_amount_alignment": "x2",
        "transaction_amount_x1_range": [0, 10000],
        "transaction_amount_x2_range": [0, 10000],
        "transaction_amount_invert_headers": ["Debit"],
        "transaction_amount_invert_alignment": "x2",
        "transaction_amount_invert_x1_range": [0, 10000],
        "transaction_amount_invert_x2_range": [0, 10000],
        "transaction_amount_invert": false,

        "transaction_balance_formats": ["format4"],
        "transaction_balance_headers": ["Balance"],
        "transaction_balance_alignment": "x2",
        "transaction_balance_x1_range": [0, 10000],
        "transaction_balance_x2_range": [0, 10000],
        "transaction_balance_invert": false
    }



How Parsing Works
-----------------
The first step in parsing is to extract all text elements from the PDF file along with their
positions on the page. The Transtractor then sequentially reads though each item and extracts
information based on sequential, positional, and formatting rules defined in the configuration 
file. 

For a view of what the Transtractor "sees" when parsing a PDF file, you can extract the PDF into
`layout text` by:

.. code-block:: python

    from transtractor import Parser
    parser = Parser()
    parser.layout('test1.pdf', 'test1_layout.txt')

The first few lines of the resulting `test1_layout.txt` file will look like:

.. code-block:: text

    [Page 0]
    ["Gravy Toast Bank",72,160,49,37,49]
    ["Fake Monthly Statement",77,238,89,75,89]
    ["Statement Period:",77,173,119,107,119]["1 Jan 2025 to 31 Jan 2025",269,411,119,107,119]
    ["Opening balance:",77,171,134,122,134]["$50,000.00 CR",269,349,134,122,134]
    ["Closing balance:",77,166,149,137,149]["$11,663.82 CR",269,350,149,137,149]
    ["Account number:",77,168,164,152,164]["1234 5678 9123 4567",269,385,164,152,164]
    ["Transaction Details",77,206,201,187,201]
    ["Date",77,103,222,210,222]["Description",149,216,222,210,222]["Debit",299,329,222,210,222]["Credit",365,400,222,210,222]["Balance",457,503,222,210,222]
    ["01 Jan",77,113,240,228,240]["Transaction 1",149,222,240,228,240]["50,000.00",346,400,240,228,240]
    ["Transaction 2",149,222,257,245,257]["1,000.00",282,329,257,245,257]
    ["Transaction 3",149,222,273,261,273]["10,000.00",275,329,273,261,273]
    ["Transaction 4",149,222,290,278,290]["1,350.00",353,400,290,278,290]["90,350 CR",445,503,290,278,290]
    ["03 Jan",77,113,307,295,307]["Transaction 5",149,222,307,295,307]["530.99",292,329,307,295,307]

Each text element is represented as ["text",x1,x2,y1,y2,y1_bin], where `text` is the extracted text,
`x1` and `x2` are the horizontal positions of the start and end of the text, and `y1` and `y2` are the
bottom and top vertical positions of the text. `y1_bin` is the vertical position of line that 
the text is assigned to.


Format Parameters
-----------------
The Transtractor uses pattern recognition and additional logic to parse amounts and dates
from text into a standardised format. Applicable formats are specified in the configuration
file (e.g., *_formats* fields). The formats currently supported are listed below. Contact 
the project maintainers if you need additional formats, or submit a pull request with your
contributions (see below).


Amount/Balance Formats
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
All amount or balance formats convert text into a decimal number with two decimal places. The following
formats are supported:


.. list-table::
    :header-rows: 1
    :widths: 20 80

    * - Label
      - Examples
    * - ``format1``
      - "1,234.56" → 1234.56, "-1,234.56" → -1234.56, "1,234.56-" → -1234.56
    * - ``format2``
      - "-$1,234.56" → -1234.56, "$1,234.56" → 1234.56, "$1,234.56-" → -1234.56
    * - ``format3``
      - "$1,234.56 CR" → 1234.56, "-$1,234.56 CR" → -1234.56, "$1,234.56 DR" → -1234.56
    * - ``format4``
      - "1,234.56 CR" → 1234.56, "-1,234.56 CR" → -1234.56, "1,234.56 DR" → -1234.56
    * - ``format5``
      - "nil" → 0.00, "Nil" → 0.00
    * - ``format6``
      - "- $1,234" → -1234.00, "+ $1,234" → 1234.00
    * - ``format7``
      - "-$1,234.56⧫" → -1234.56, "$1,234.56⧫" → 1234.56, "$1,234.56-⧫" → -1234.56

Formats are sensitive to spacing and comma separation, but generally not case sensitive.


Date Formats
~~~~~~~~~~~~~~~~~~~~~~~~
Date formats convert text into the standard ISO format of YYYY-MM-DD. The following
formats are supported:

.. list-table::
    :header-rows: 1
    :widths: 20 80

    * - Label
      - Examples
    * - ``format1``
      - "24 Mar" → XXXX-03-24, "24 mar" → XXXX-03-24, "24 March" → XXXX-03-24
    * - ``format2``
      - "24 march 2025" → 2025-03-24, "24 Mar 2025" → 2025-03-24
    * - ``format3``
      - "mar 24, 2025" → 2025-03-24, "March 24, 2025" → 2025-03-24
    * - ``format4``
      - "24/3/2020" → 2020-03-24, "24/3/2020" → 2020-03-24
    * - ``format5``
      - "24/3/25" → 2025-03-24, "24/03/25" → 2025-03-25
    * - ``format6``
      - "3/24" → XXXX-03-24, "03/24" → XXXX-03-24
    * - ``format7``
      - "24-03-2023" → 2023-03-24, "24-3-2023" → 2023-03-24, "24-03-23" → 2023-03-24, "24-3-23" → 2023-03-24
    * - ``format8``
      - "03-24-2023" → 2023-03-24, "3-24-2023" → 2023-03-24, "03-24-23" → 2023-03-24, "3-24-23" → 2023-03-24
    * - ``format9``
      - "03/24/2023" → 2023-03-24, "3/24/2023" → 2023-03-24, "03/24/23" → 2023-03-24, "3/24/23" → 2023-03-24
    * - ``format10``
      - "Mar 24" → XXXX-03-24, "March 24" → XXXX-03-24, "March 4" → XXXX-03-04
    * - ``format11``
      - "Mar 24, 2023-Apr 24, 2023" → 2023-03-24, "March 1, 2020-March 31, 2020" → 2020-03-01
    * - ``format12``
      - "2023/03/24" → 2023-03-24, "2023/3/24" → 2023-03-24
    * - ``format13``
      - "2023-03-24" → 2023-03-24, "2023-3-24" → 2023-03-24
    * - ``format14``
      - "03/24/2023*" → 2023-03-24, "3/24/2023*" → 2023-03-24, "03/24/23*" → 2023-03-24, "3/24/23*" → 2023-03-24
    * - ``format15``
      - "Jul 1 - Jul 31, 2026" → 2026-07-01
    * - ``format16``
      - "1 Jul 26" → 2026-07-01, "01 July 26" → 2026-07-01

Formats with a "XXXX" year will infer the year based on the statement start date.


Add New Formats
~~~~~~~~~~~~~~~~~
New formats must be added to the Rust source code *src/formats/amount* or 
*src/formats/date* directory. Add your new format parser as a new module and update the
register it in the mod.rs file. Please follow the existing code structure and include unit tests
for your new format. Re-compile with Maturin to apply changes.

Pull requests are welcome!


Extraction Parameters
---------------------
The parser extracts the *Account Number*, *Start Date*, *Opening and Closing Balances*,
and the tabulated *Transactions* from the statement. The *Transactions* must include at least
the *Date*, *Description*, and *Amount* fields. *Balance* is optional. Implicit values are filled
automatically (e.g., missing balances, shared dates).The extraction parameters used in the 
configuration file are described below.


General Statement Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify the statement and configuration setting,
and control any pre-processing of the text before extraction begins.

*key*
*********************
Descriptive unique identifier for the configuration file. This must follow the format 
``<country_code>__<bank_code>__<account_type>__<version>``. The country_code
must be a valid lowercase 
`ISO 3166-1 alpha-2 country code <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>`_. 
The bank_code is ideally the stock ticker symbol or a commonly used abbreviation for the bank, 
but can be another short lowercase string. The account_type is a descriptive short lowercase
string such as "debit", "credit_card", or "loan". The version is an integer starting from 1
that is incremented for each new version of the configuration file.

*bank_name*
*********************
Full name of the bank. Max. 100 characters.

*account_type*
*********************
Must be one of:

- "Checking"
- "Savings"
- "Credit Card"
- "Loan"
- "Mortgage"
- "Investment"
- "Mixed"
- "Other"

*account_terms*
*********************
List of terms that distinguish this statement from the statements
from other banks or account types from the same bank. The statement
must have all these terms present to be considered a match. If a statement
matches the terms of multiple configuration files, then all configuration files
will be tried in sequence until one successfully parses the statement.

*account_examples*
************************
List of example account product names that this configuration file is intended to support.
Many banks will often have multiple account products with similar statement formats. For example,
the Commonwealth Bank of Australia's "Smart Access", "Streamline" and "Everyday Offset" accounts 
use the same statement format.


Account Number Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the account number from the statement.

*account_number_terms*
*******************************
List of text terms that appear before or above the account number. This will prime
the parser to start scanning for an account number satisfying one of the 
*account_number_patterns*. The parser will stop trying to find the account number once
it is set. The parser only requires one of these terms to be present to start searching
for the account number.

*account_number_trigger_count*
**********************************************
Integer value specifying the number of times a term from *account_number_terms* must be found
before the parser will start searching for an account number. Specifying zero will cause the 
parser to start searching for an account number without requiring any of the 
*account_number_terms* to be present.

*account_number_patterns*
****************************************
List of regular expression patterns that match the account number format. The parser 
will try to match each pattern in sequence until one is found.

*account_number_alignment*
*************************************
Specifies the alignment of the account number relative to the *account_number_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
account number must be horizontally aligned with the *account_number_terms*. 
If set to "", then no alignment checking will be performed and the first matching
account number found after the *account_number_terms* will be used.

*account_number_alignment_tol*
*****************************************
Integer value specifying the tolerance (in points) for alignment checking of the
account number. For example, if *account_number_alignment* is "y1" and this value is 5,
then the *y1* position of the account number must be within 5 points of the *y1* position
of the *account_number_terms*.


Opening Balance Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the opening and closing balances
from the statement.

*opening_balance_terms*
*************************************
List of text terms that appear before or above the opening balance. This will prime
the parser to start scanning for the opening balance. The parser will stop trying to find
the opening balance once it is set. The parser only requires one of these terms to be present
to start searching for the opening balance.

*opening_balance_trigger_count*
**********************************************
Integer value specifying the number of times a term from *opening_balance_terms* must be found
before the parser will start searching for an opening balance. Specifying zero will cause the 
parser to start searching for an opening balance without requiring any of the *opening_balance_terms* 
to be present.

*opening_balance_formats*
****************************************
List of amount formats (see above) that the opening balance may be in. The parser will try to
parse each format in sequence until one is successful.

*opening_balance_alignment*
***************************************
Specifies the alignment of the opening balance relative to the *opening_balance_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
opening balance must be horizontally aligned with the *opening_balance_terms*. 
If set to "", then no alignment checking will be performed and the first matching
opening balance found after the *opening_balance_terms* will be used.

*opening_balance_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
opening balance. For example, if *opening_balance_alignment* is "y1" and this value is 5,
then the *y1* position of the opening balance must be within 5 points of the *y1* position
of the *opening_balance_terms*.

*opening_balance_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted opening balance. This is
often useful for loan or credit card statements where the opening balance is presented as a
positive value despite it being a liability.


Closing Balance Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the closing balance from the statement.

*closing_balance_terms*
*************************************
List of text terms that appear before or above the closing balance. This will prime
the parser to start scanning for the closing balance. The parser will stop trying to find
the closing balance once it is set. The parser only requires one of these terms to be present
to start searching for the closing balance.

*closing_balance_trigger_count*
**********************************************
Integer value specifying the number of times a term from *closing_balance_terms* must be found
before the parser will start searching for a closing balance. Specifying zero will cause the
parser to start searching for a closing balance without requiring any of the *closing_balance_terms*
to be present.

*closing_balance_formats*
****************************************
List of amount formats (see above) that the closing balance may be in. The parser will try to
parse each format in sequence until one is successful.

*closing_balance_alignment*
***************************************
Specifies the alignment of the closing balance relative to the *closing_balance_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
closing balance must be horizontally aligned with the *closing_balance_terms*. 
If set to "", then no alignment checking will be performed and the first matching
closing balance found after the *closing_balance_terms* will be used.

*closing_balance_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
closing balance. For example, if *closing_balance_alignment* is "y1" and this value is 5,
then the *y1* position of the closing balance must be within 5 points of the *y1* position
of the *closing_balance_terms*.

*closing_balance_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted closing balance. This is
often useful for loan or credit card statements where the closing balance is presented as a
positive value despite it being a liability.

*closing_balance_set_from_last_transaction*
*************************************
Boolean value (default *false*) specifying whether to set the closing balance to the running
balance of the last extracted transaction, rather than using the value parsed from the statement
via the other *closing_balance_* parameters. This is useful when the statement does not state a
closing balance. Only set this to *true* if the statement provides running transaction balances.


Start Date Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the statement start date from the statement.
This date is used to infer missing years in transaction dates.

*start_date_terms*
*************************************
List of text terms that appear before or above the statement start date. This will prime
the parser to start scanning for the start date. The parser will stop trying to find
the start date once it is set. The parser only requires one of these terms to be present
to start searching for the start date.

*start_date_trigger_count*
**********************************************
Integer value specifying the number of times a term from *start_date_terms* must be found
before the parser will start searching for a start date. Specifying zero will cause the
parser to start searching for a start date without requiring any of the *start_date_terms*
to be present.

*start_date_formats*
****************************************
List of date formats (see above) that the start date may be in. The parser will try to
parse each format in sequence until one is successful.

*start_date_alignment*
***************************************
Specifies the alignment of the start date relative to the *start_date_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
start date must be horizontally aligned with the *start_date_terms*. 
If set to "", then no alignment checking will be performed and the first matching
start date found after the *start_date_terms* will be used.

*start_date_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
start date. For example, if *start_date_alignment* is "y1" and this value is 5,
then the *y1* position of the start date must be within 5 points of the *y1* position
of the *start_date_terms*.

Transaction Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the transaction table from the statement.

*transaction_terms*
*************************************
List of text terms that indicate the start of the transaction table. The parser will start
looking for transactions after these terms are found. The parser only requires one of these
terms to be present to start searching for transactions.

*transaction_trigger_count*
**********************************************
Integer value specifying the number of times a term from *transaction_terms* must be found
before the parser will start searching for transactions. Specifying zero will cause the
parser to start searching for transactions without requiring any of the *transaction_terms*
to be present.

*transaction_terms_stop*
*************************************
List of text terms that indicate the end of the transaction table. The parser will stop
looking for transactions once these terms are found. The parser only requires one of these
terms to be present to stop searching for transactions.

*transaction_trigger_stop_count*
**********************************************
Integer value specifying the number of times a term from *transaction_terms_stop* must be found
before the parser will stop searching for transactions. Specifying zero will cause the
parser to stop searching for transactions without requiring any of the *transaction_terms_stop*
to be present.

*transaction_formats*
****************************************
List of expected transaction field arrangements. Each arrangement is a list of field names
from the set: "date", "description", "amount", "balance". This allows the parser to know when
to start and stop reading fields for each transaction, and recognised when a transaction is 
complete.

*transaction_start_date_required*
******************************************
Boolean value specifying whether the start date is required for parsing transactions. Set as true
if transaction dates do not specify the year and need to be inferred from the statement start date.

*transaction_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
transaction fields and the field headers.

*transaction_date_formats*
****************************************
List of date formats (see above) that transaction dates may be in. The parser will try to
parse each format in sequence until one is successful.

*transaction_date_headers*
****************************************
List of text headers that identify the transaction date column. The parser will use these
to identify the horizontal position of the date field in the transaction table.

*transaction_date_alignment*
****************************************
Specifies the alignment of the transaction date field relative to the *transaction_date_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_date_x1_range* and *transaction_date_x2_range*
****************************************
Two-element inclusive ranges for the transaction date's x1 and x2 coordinates, respectively.
Both default to ``[0, 10000]``. When a date header is configured and found, its position
overrides the range selected by *transaction_date_alignment*. Without date headers, alignment
does not affect matching; these coordinate ranges are used directly.

*transaction_description_headers*
****************************************
List of text headers that identify the transaction description column. The parser will use these
to identify the horizontal position of the description field in the transaction table.

*transaction_description_alignment*
****************************************
Specifies the alignment of the transaction description field relative to the *transaction_description_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_description_x1_range* and *transaction_description_x2_range*
****************************************
Two-element inclusive ranges for the transaction description's x1 and x2 coordinates, respectively.
Both default to ``[0, 10000]``. When a description header is configured and found, its position
overrides the range selected by *transaction_description_alignment*. Without description headers,
alignment does not affect matching; these coordinate ranges are used directly.

*transaction_description_exclude*
****************************************
List of regex patterns to identify and remove unwanted text in transaction descriptions. This is
useful for filtering out recurring header or footer text that may appear in the transaction
descriptions, dot leaders, or other unwanted text.

*transaction_amount_formats*
****************************************
List of amount formats (see above) that transaction amounts may be in. The parser will try to
parse each format in sequence until one is successful.

*transaction_amount_headers*
****************************************
List of text headers that identify the transaction amount column. The parser will use these
to identify the horizontal position of the amount field in the transaction table. If there are
separate debit and credit columns, then set this to the credit column header.

*transaction_amount_alignment*
****************************************
Specifies the alignment of the transaction amount field relative to the *transaction_amount_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_amount_x1_range* and *transaction_amount_x2_range*
****************************************
Two-element inclusive ranges for the transaction amount's x1 and x2 coordinates, respectively.
Both default to ``[0, 10000]``. When an amount header is configured and found, its position
overrides the range selected by *transaction_amount_alignment*. Without amount headers, alignment
does not affect matching; these coordinate ranges are used directly.

*transaction_amount_invert_headers*
****************************************
List of text headers that identify transaction amount columns where the sign needs to be inverted.
For example, if there are separate debit and credit columns, then set this to the debit column header.

*transaction_amount_invert_alignment*
****************************************
Specifies the alignment of the transaction amount field relative to the *transaction_amount_invert_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_amount_invert_x1_range* and *transaction_amount_invert_x2_range*
****************************************
Two-element inclusive ranges for x1 and x2 coordinates of amounts in the inverted column,
respectively. Both default to ``[0, 10000]``. When an invert header is configured and found, its
position overrides the range selected by *transaction_amount_invert_alignment*. Without invert
headers, alignment does not affect matching.

*transaction_amount_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted transaction amounts. This is
often useful for loan or credit card statements where debits are presented as positive values
despite being liabilities.

*transaction_balance_formats*
****************************************
List of amount formats (see above) that transaction balances may be in. The parser will try to
parse each format in sequence until one is successful. Leave empty if transaction balances are not
present in the statement.

*transaction_balance_headers*
****************************************
List of text headers that identify the transaction balance column. The parser will use these
to identify the horizontal position of the balance field in the transaction table. Leave empty
if transaction balances are not present in the statement.

*transaction_balance_alignment*
****************************************
Specifies the alignment of the transaction balance field relative to the *transaction_balance_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned). Cannot be left empty.

*transaction_balance_x1_range* and *transaction_balance_x2_range*
****************************************
Two-element inclusive ranges for the transaction balance's x1 and x2 coordinates, respectively.
Both default to ``[0, 10000]``. When a balance header is configured and found, its position
overrides the range selected by *transaction_balance_alignment*. Without balance headers,
alignment does not affect matching; these coordinate ranges are used directly.

*transaction_balance_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted transaction balances. This is
often useful for loan or credit card statements where balances are presented as positive values
despite being liabilities.

*transaction_balance_ignore*
*************************************
Boolean value specifying whether to ignore and regenerate extracted transaction balances. 
This is useful when reordering transactions by date will cause errors to surface due from running
balance validation checks.

Testing Your Configuration
--------------------------------------
Once you have created your configuration file, you can test it by loading it into the
Transtractor parser and parsing a sample statement:

.. code-block:: python

    from transtractor import Parser

    # Initialise parser with your configuration file
    parser = Parser()
    parser.load('your_config_file.json')
    parser.parse('sample_statement.pdf').to_csv('sample_statement.csv')

Even better, try it out against all your bank statements to ensure it works across
multiple years and is tolerant of edge cases:

.. code-block:: python

    from transtractor import Parser

    parser = Parser()
    parser.load('your_config_file.json')
    parser.test('directory_containing_statements', 'test_results.csv')

This will recursively parse all PDF statements in the directory and sub-directories, and
output a CSV file with the results. Review the results to ensure all statements were parsed
correctly.

Troubleshooting
----------------------
Here are some common issues you may encounter when creating configuration files,
and how to resolve them.

Zero-Balances
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Sometimes statements will show zero balances as "nil", "zero", or similar text.
Ensure you include *format5* in the relevant *_formats* fields to handle these cases.
This is a common case for you first statement of a new account. 

It is also recommended that amount formats *format3* and *format4* are specified 
alongside *format1* and *format2*, respectively, to handle zero amounts/balances without a 
trailing "CR" or "DR". The *transaction_alignment_tol* may need to be relaxed since 
these balances may be offset from the *transaction_balance/amount_headers*.

Hidden Characters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
PDFs may contain characters that are not visible when viewing the statement, but
are extracted by the parser. These hidden characters can interfere with pattern matching.
To identify hidden characters, extract the layout text using the `layout` method
(as described above) and inspect the text around the problematic areas. You may need
to adjust your regex patterns to account for these hidden characters.

Missing Date or Amount Formats
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
The Transtractor is still growing its library of supported date and amount formats. If you encounter
a date or amount format that is not recognised, you may need to add a new format parser
to the Rust source code (as described above). Contact the project maintainers if you need
assistance with this process.


Contributing Your Configuration
--------------------------------------
If you have created a well-tested configuration file for a bank or account type that is not
currently supported, please consider contributing it to the project. Follow the 
`Contributor Guide <https://github.com/weberdak/transtractor-lib/blob/main/CONTRIBUTING.md>`_
in the GitHub repository.
