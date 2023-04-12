# demo-secrets
Skeleton project demonstrating the use of [git secrets](https://git-secret.io/) on the Engi network.


## Dependencies
 - git-secret
 - gnupg

## Setup

- Make sure to generate a gpg key, if you don't have one. See [here](https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key) for instructions.
- Fetch ENGI's public gpg key [here]()
- Import ENGI key into your gpg keyring: `gpg --import engi_public_key.txt`
- Initialize git secrets in your repo: `git secret init`
- Add ENGI to the list of allowed decryptors: `git secret tell circleci@engi.network`
- Add yourself to the list of allowed decryptors: `git secret tell your.email@somewhere.special`
- Add your secrets to the store, for example a test-data.json file: `git secret add test-data.json`


```bash
$ git secret whoknows -l
circleci@engi.network (expires: 2024-12-05)
```
