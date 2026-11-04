Tilføj denne linje til din applikations Gemfile:

```ruby
gem 'fastcomments'
```

Og udfør derefter:

```bash
bundle install
```

Eller installer det selv med:

```bash
gem install fastcomments
```

### Bibliotekets indhold

Dette bibliotek indeholder den genererede API-klient og SSO-værktøjerne, der gør det lettere at arbejde med API'en.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Offentlige vs. sikrede API'er

For API-klienten er der tre klasser: `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder API-kald,
der kan foretages direkte fra en browser/mobilenhed/os. uden autentificering. `ModerationApi` indeholder de metoder, der driver moderator-dashboardet.

`ModerationApi` giver en omfattende suite af live- og hurtige moderations-API'er. Hver `ModerationApi`-metode accepterer en `sso`-parameter og kan autentificere via SSO eller en FastComments.com-session-cookie.