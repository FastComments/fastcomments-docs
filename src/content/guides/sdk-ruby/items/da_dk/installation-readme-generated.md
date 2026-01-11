Tilføj denne linje til din applikations Gemfile:

```ruby
gem 'fastcomments'
```

Og kør derefter:

```bash
bundle install
```

Eller installer den selv som:

```bash
gem install fastcomments
```

### Bibliotekets indhold

Dette bibliotek indeholder den genererede API-klient og SSO-værktøjer, der gør det lettere at arbejde med API'et.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Offentlige vs beskyttede API'er

For API-klienten er der to klasser, `DefaultApi` og `PublicApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder API-opkald
der kan foretages direkte fra en browser/mobil enhed/etc uden autentificering.
---