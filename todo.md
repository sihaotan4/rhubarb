study python FFI - being able to call functions on the SetRegistry in python
maybe the way to think about this is like a dataframe construct
- loading data into it
- a bit like an in memory DB
- and then providing a set of APIs which can act on the construct and return data
- in this case our API is a query language like this (A OR ((C AND B) OR D)) 

schema changes
- push or pull?
- how to handle changes in sets - do we lazily evaluate sets?
- say a table is removed from a particular schema or added to a particular schema
- an employee is shifted from one division to another division
- how do we force a refresh of the data structure and maybe the access matrix (is this coupled or decoupled)

frontend 
- which business team is more likely to use this? are they technical
- query text completion (like intellisense)
- set visualizer for those not so inclined (duo screen for asset side and user side)