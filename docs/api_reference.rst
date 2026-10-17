API Reference
=============

This page documents the public API for Transtractor.


Parser
------

The main entry point for parsing bank statement PDFs.

.. autoclass:: transtractor.Parser
   :members:
   :undoc-members:
   :show-inheritance:

   .. automethod:: __init__


StatementData
-------------

Represents the parsed bank statement data, including account information and transactions.

.. autoclass:: transtractor.StatementData
   :members:
   :undoc-members:
   :show-inheritance:

   .. automethod:: __init__


Transaction
-----------

Represents an individual transaction within a bank statement.

.. autoclass:: transtractor.Transaction
   :members:
   :undoc-members:
   :show-inheritance:

   .. automethod:: __init__
