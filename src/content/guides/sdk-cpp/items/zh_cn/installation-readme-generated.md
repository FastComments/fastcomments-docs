### Install Dependencies

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Building from Source

```bash
mkdir build
cd build
cmake ..
make
```

### Installing

```bash
sudo make install
```

### Library Contents

This library contains the generated API client and the SSO utilities to make working with the API easier.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Public vs Secured APIs

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains
methods that can be made directly from a browser/mobile device/etc without authentication. The `ModerationApi` contains methods that power the moderator dashboard - listing,
counting, searching, exporting and pulling logs for comments, moderation actions (remove/restore, flag, set review/spam/approval status, adjust votes, reopen/close threads),
bans (ban from a comment, undo bans, pre-ban summaries, ban status and preferences, banned-user counts), and badges & trust (award/remove badges, manual badges, get/set trust
factor, user internal profile). Every `ModerationApi` method accepts an `sso` parameter so the call is performed on behalf of an SSO-authenticated moderator.