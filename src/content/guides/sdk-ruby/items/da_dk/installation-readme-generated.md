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

### Biblioteksindhold

Dette bibliotek indeholder den genererede API‑klient og SSO‑værktøjerne, der gør arbejdet med API’en lettere.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Offentlige vs sikrede API’er

For API‑klienten findes der tre klasser, `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API‑nøgle, og `PublicApi` indeholder API‑kald, der kan foretages direkte fra en browser/mobil enhed osv. uden autentifikation. `ModerationApi` indeholder metoderne, der driver moderator‑dashboardet.

`ModerationApi` leverer en omfattende suite af live‑ og hurtige moderations‑API’er. Hver `ModerationApi`‑metode accepterer en `sso`‑parameter og kan autentificere via SSO eller en FastComments.com‑session‑cookie.