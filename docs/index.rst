The Transtractor
================

**A Very Good Extractor of Bank Statement PDFs**

The Transtractor (Transaction Extractor) is a high-performance, rules-based library for
extracting structured transaction data from PDF bank statements issued by different banks
and financial institutions. It converts varied statement layouts into validated transaction 
tables with consistent dates, descriptions, amounts, balances.

The parsing engine is built with Rust and is available through a Python API, browser-based
WebAssembly bindings, and an `online interface <https://www.transtractor.net/>`_.

.. image:: https://img.shields.io/pypi/v/transtractor
   :target: https://pypi.org/project/transtractor/
   :alt: PyPI version

.. image:: https://img.shields.io/pypi/status/transtractor
   :target: https://pypi.org/project/transtractor/
   :alt: Development Status

.. image:: https://github.com/weberdak/transtractor-lib/actions/workflows/tests.yml/badge.svg
   :target: https://github.com/weberdak/transtractor-lib/actions
   :alt: Tests

.. image:: https://codecov.io/gh/transtractor/transtractor-lib/branch/main/graph/badge.svg
   :target: https://codecov.io/gh/transtractor/transtractor-lib
   :alt: codecov

.. image:: https://img.shields.io/github/license/transtractor/transtractor-lib
   :target: https://github.com/weberdak/transtractor-lib/blob/main/LICENSE
   :alt: License


Quick Start
-----------

Install from PyPI:

.. code-block:: bash

   pip install transtractor

To parse a bank statement PDF and convert it to CSV:

.. code-block:: python

   from transtractor import Parser

   # Initialise parser
   parser = Parser()

   # Convert PDF to CSV
   parser.parse('statement.pdf').to_csv('statement.csv')

Writes a CSV of the following standard format regardless of the original statement layout:

.. code-block:: text

   date,description,amount,balance,account_number
   2025-01-01,Transaction 1,50000.0,100000.0,1234 5678 9123 4567
   2025-01-01,Transaction 2,-1000.0,99000.0,1234 5678 9123 4567
   2025-01-01,Transaction 3,-10000.0,89000.0,1234 5678 9123 4567
   2025-01-01,Transaction 4,1350.0,90350.0,1234 5678 9123 4567
   2025-01-03,Transaction 5,-530.99,89819.01,1234 5678 9123 4567
   2025-01-03,Transaction 6,1532.55,91351.56,1234 5678 9123 4567
   2025-01-04,Transaction 7,-568.01,90783.55,1234 5678 9123 4567
   2025-01-04,Transaction 8,-23.56,90759.99,1234 5678 9123 4567
   ...

All columns are included in the CSV output, with the date in ISO format (YYYY-MM-DD), 
the description as text, and the amount and balance as decimal numbers. Negative amounts 
indicate debits and positive amounts indicate credits.

Pooling Across Multiple PDFs and Accounts
-----------------------------------------

The following snippet demonstrates how to batch-extract information from all your PDF 
statements in a single directory:

.. code-block:: python

   import csv
   from pathlib import Path

   from transtractor import Parser

   # Paths to the PDF directory and output CSV file
   pdf_directory = Path("path/to/your/pdf/directory")
   output_csv = Path("output.csv")

   parser = Parser()
   transactions = set()

   # Find and extract PDFs in directory and subdirectories
   for pdf_path in sorted(pdf_directory.rglob("*.pdf")):
      print(f"Parsing PDF: {pdf_path}")
      statement_data = parser.parse(str(pdf_path))
      transactions.update(statement_data.transactions)

   # Arrange newest to oldest
   ordered_transactions = sorted(
      transactions,
      key=lambda transaction: (
         transaction.date,
         transaction.date_index,
         transaction.account_number,
      ),
      reverse=True,
   )

   # Write the ordered transactions to the output CSV file
   with output_csv.open("w", newline="", encoding="utf-8") as csv_file:
      writer = csv.writer(csv_file)
      writer.writerow(("date", "description", "amount", "balance", "account_number"))
      for transaction in ordered_transactions:
         writer.writerow(
               (
                  transaction.date,
                  transaction.description,
                  transaction.amount,
                  transaction.balance,
                  transaction.account_number,
               )
         )

Web Interface
-------------
A user-friendly web interface for bulk extraction is available at 
`transtractor.net <https://transtractor.net>`_.

Supported Banks
---------------
See the :doc:`supported statements <supported_statements>` page for a full list of supported banks and statement formats.
If your bank is not supported, create your own configuration by following the :doc:`guidelines <configuration>` and 
load it into the parser:

.. code-block:: python

   from transtractor import Parser

   parser = Parser()
   parser.load('path/to/your_bank_config.json')
   parser.parse('your_statement.pdf').to_csv('output.csv')


Documentation
-------------

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   installation
   supported_statements
   configuration
   api_reference


Community & Support
-------------------

* **Website**: `transtractor.net <https://transtractor.net>`_
* **PyPI**: `transtractor <https://pypi.org/project/transtractor/>`_
* **GitHub Repository**: `weberdak/transtractor-lib <https://github.com/weberdak/transtractor-lib>`_
* **Contributions**: `weberdak/transtractor-lib/CONTRIBUTING.md <https://github.com/weberdak/transtractor-lib/blob/main/CONTRIBUTING.md>`_


License
-------

The Transtractor is open source software licensed under the MIT License.
