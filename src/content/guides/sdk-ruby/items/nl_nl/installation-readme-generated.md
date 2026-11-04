Voeg deze regel toe aan de Gemfile van je applicatie:

```ruby
gem 'fastcomments'
```

Voer daarna uit:

```bash
bundle install
```

Of installeer het zelf als volgt:

```bash
gem install fastcomments
```

### Inhoud van de Library

Deze library bevat de gegenereerde API-client en de SSO-hulpprogramma's om het werken met de API eenvoudiger te maken.

- [API Client Library Documentatie](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Publieke versus Beveiligde API's

Voor de API-client zijn er drie klassen: `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API-sleutel vereisen, en `PublicApi` bevat API-aanroepen
die direct vanuit een browser/mobiel apparaat/enz. kunnen worden gedaan zonder authenticatie. De `ModerationApi` bevat de methoden die het moderatordashboard aandrijven.

De `ModerationApi` biedt een uitgebreide reeks live en snelle moderatie-API's. Elke `ModerationApi`-methode accepteert een `sso`-parameter en kan authenticeren via SSO of een FastComments.com-sessiecookie.