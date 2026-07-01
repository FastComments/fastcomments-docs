Voeg deze regel toe aan het Gemfile van je applicatie:

```ruby
gem 'fastcomments'
```

Voer vervolgens uit:

```bash
bundle install
```

Of installeer het zelf met:

```bash
gem install fastcomments
```

### Library Contents

Deze bibliotheek bevat de gegenereerde API‑client en de SSO‑hulpmiddelen om het werken met de API gemakkelijker te maken.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

Voor de API‑client zijn er drie klassen: `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API‑sleutel vereisen, en `PublicApi` bevat API‑aanroepen die rechtstreeks vanuit een browser/mobiel apparaat/etc. kunnen worden uitgevoerd zonder authenticatie. De `ModerationApi` bevat de methoden die het moderator‑dashboard mogelijk maken.

De `ModerationApi` biedt een uitgebreide reeks realtime en snelle moderatie‑API’s. Elke `ModerationApi`‑methode accepteert een `sso`‑parameter en kan authenticeren via SSO of een FastComments.com‑sessiecookie.