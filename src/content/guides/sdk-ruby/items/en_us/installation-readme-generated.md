Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Library Contents

This library contains the generated API client and the SSO utilities to make working with the API easier.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

For the API client, there are three classes, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contains api calls
that can be made directly from a browser/mobile device/etc without authentication. The `ModerationApi` contains the methods that power the moderator dashboard.

The `ModerationApi` covers comment moderation (list, count, search, logs, export), moderation actions (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread),
bans (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), and badges & trust (award/remove badge, manual badges, get/set trust factor, user internal profile).
Each `ModerationApi` method accepts an `sso` parameter so the request can be made on behalf of an SSO-authenticated moderator.