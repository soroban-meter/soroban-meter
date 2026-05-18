# soroban-meter
Soroban's fee model is multidimensional — CPU instructions, read/write bytes, ledger entry access, rent, and events are all metered independently. A transaction fails if refundable fees don't cover actual usage at execution time. Yet no first-class tool exists that surfaces this breakdown during development, where you can actually act on it.
