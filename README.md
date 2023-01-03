bidwasm
&middot;
![GitHub](https://img.shields.io/github/license/giorgionocera/bidwasm)
![GitHub last commit](https://img.shields.io/github/last-commit/giorgionocera/bidwasm)
=====

A smart contract for bidding procedure 

## 🎰 What is bidwasm?
It is a smart contract to enable bidding procedure functionalities written in 
the Rust language to run inside a Cosmos SDK module on all chains enabling it.
It is my final assignment for the CosmWasm Academy project.

## 🎯 Requirements (from the Assignment)
At instantion, a user opens a bid for some offchain commodity. Bid will be 
happening using only single native token (e.g., ATOM). Contract owner is 
optionally provided by its creator - if missing, contract creator is considered
its owner.

After contract is instantiated, any user other than the contract owner can 
raise his bid by sending tokens to the contract with the `bid {}` message. When
the message is called, part of the tokens send are immediately considered 
bidding commission and should be transferred to contract owner. It is up to you
to figure out how to calculate commission.

The total bid of the user is considered to be a sum of all bids performed minus
all the commissions. When user raises his bid, it should success only if his 
total bid is the highest of all other users bids. If it is less or the same as
the highest, bidding should fail.

Owner can `close {}` the bidding at any time. When the bidding is closed, 
address with the highest total bid is considered the bidding winner. The whole bidding of his is transferred to the contract owner.

After the bidding is closed, everyone who bid and didn't win the bidding, can
`retract {}` all his funds. Additionally the retract message should have an 
optional friend receiver being an address where the sender biddings should be 
send. So `retract {}` sends all senders bids (minus commissions) to his 
account. The `retract { "receiver": "addr" }` should send all the sender bids 
to the `"addr"` account.

Additionally - all the information kept on the contract should be queryable in
reasonable manner. The most important queries are: 
- the given addr total bid;
- the highest bid at the current time (who and how much);
- if the bidding is closed;
- who won the bid (if it is closed).

The contract should contain some tests using multitests framework, but I do not
expect any particular coverage - 2-3 main flow tests should be enough.

## 🗣 Example
There is the bidding created at `bidding_contract` address. **Alex** is sending
`bid {}` message with **15 atoms**. The highest bid right now is *15 atoms by 
Alex*. Now **Ann** is sending `bid {}` message with **17 atoms**. The highest 
bid is *17 atoms by Ann*, and *total bid by alex is 15 atoms*. Now **Ann** is 
sending another `bid {}` message with **2 atoms**. Now the highest bid is *19 
atoms by Ann*, and *total of Alex is 15 atoms*. Then **Alex** sends `bid {}`
message with **1 atom** - this message fails, as it would leave **Alex** at *16 
atoms* bid total, which is not the highest right now. He has to send more than 
5 atoms. Alex sends another `bid {}` with **5 atoms**. It makes the highest bid
being *20 atoms by Alex*, and *Ann has total of 19 atoms bid*. The `close {}` 
is send by **contract owner** - **Alex wins** the bid, **20 atoms are send to 
bid owner** from `bidding_contract`. **Ann can claim her atoms back** calling
`retract {}` message, optionally putting a receiver field there to point where
funds should be send back to.

## 📜 License

Copyright © 2023 Giorgio Nocera. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
