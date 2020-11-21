# Relationship between code advance (including tests) and HUs


Work has begun on this aspect since the commit: [#3c99ae1](https://github.com/pepitoenpeligro/CloudBanking/commit/3c99ae17ad243a9c31496c395886cb273caf155c) 


The work history for this correction ends at the commit: [#ed19517](https://github.com/pepitoenpeligro/CloudBanking/commit/ed19517313d9540a886a87218a345269e1ab8c3d)


* [Updating cc.yaml](https://github.com/pepitoenpeligro/CloudBanking/commit/2a142ba08cf88a93b834955d9f2a88746e249a12#diff-3fa6b66a746b1f9567323e318eb1035d57e5cad60a2619d4e86b2c5e56c16171)
* [Adding TaskManager #c99188d](https://github.com/pepitoenpeligro/CloudBanking/commit/c99188d5af9cffe88c0fef246e6d386a9c5e6be3)
* **User**
  - [test model User](https://github.com/pepitoenpeligro/CloudBanking/commit/131f01176ffe695025f74d12471f390a90b0094d)
  - [Adding Bank Acount - HU 1](https://github.com/pepitoenpeligro/CloudBanking/commit/d850fb557b3dad6441135f75f83ac2bfe622d87b#diff-53630e1968a2dca78c40a46aa0b0cbd39e68220836815c497abf9120f29269b2)
* **Story about BankAccountController**
    - [Adding concurrent data structure](https://github.com/pepitoenpeligro/CloudBanking/commit/df5b47a0d577298179b9594246f67895c4a8fd63#diff-0de7b4973a2eba14f5c124d780043b0cc09945de92b6e2a926f841fd275b9dfa)
    - [remove login_user, test_create_new_user, create_new_user - HU 14](https://github.com/pepitoenpeligro/CloudBanking/commit/f8fda22c0cd7bbc9a2c84866bbc8b9f0af11d4f1#diff-0de7b4973a2eba14f5c124d780043b0cc09945de92b6e2a926f841fd275b9dfa)
    - [test model CloudBankingController](https://github.com/pepitoenpeligro/CloudBanking/commit/32ac4286a29fb183f2b7eddbfd3836ad1c19956d#diff-0de7b4973a2eba14f5c124d780043b0cc09945de92b6e2a926f841fd275b9dfa)
    - [test_get_user, test_erase_account, get_users, erase_account - HU 16](https://github.com/pepitoenpeligro/CloudBanking/commit/32ac4286a29fb183f2b7eddbfd3836ad1c19956d#diff-0de7b4973a2eba14f5c124d780043b0cc09945de92b6e2a926f841fd275b9dfa)
    - [test delete_bank_accoutn, delete_bank_account - HU 4](https://github.com/pepitoenpeligro/CloudBanking/commit/1cc2afb7880feacd1efdb020edf0a635dffdcf06)
* Adding Utils library
   - [Swfit and IBAN validity and unittest, test model Lib](https://github.com/pepitoenpeligro/CloudBanking/commit/a582cd286e17a76b24b818875fbd587b8ae0eb86#diff-91e6cd49b1c257b938d5283e1e0bb36d4f9154ef27e4f68ff50de0c5af7f6a55)



Additionally it has been included:
1. [GitHubAction cc.yaml.check and badge in README.md](https://github.com/pepitoenpeligro/CloudBanking/blob/master/.github/workflows/check_cc.yaml.yml)
2. [GitHubAction rust test and badge in README.md](https://github.com/pepitoenpeligro/CloudBanking/blob/master/.github/workflows/rust.yml)
3. [Travis integration and badge in README.md](https://github.com/pepitoenpeligro/CloudBanking/blob/master/.travis.yml)
4. [Source Code Documentation and badge in README.md](https://rawcdn.githack.com/pepitoenpeligro/CloudBanking/master/docs/documentation/CloudBanking/index.html )

