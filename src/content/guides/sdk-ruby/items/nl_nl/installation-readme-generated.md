Voeg deze regel toe aan het Gemfile van uw applicatie:

```ruby
gem 'fastcomments'
```

En voer vervolgens uit:

```bash
bundle install
```

Of installeer het zelf met:

```bash
gem install fastcomments
```

### Bibliotheekinhoud

Deze bibliotheek bevat de gegenereerde API-client en de SSO-hulpmiddelen om het werken met de API te vereenvoudigen.

- [Documentatie van de API-clientbibliotheek](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Publieke vs Beveiligde API's

Voor de API-client zijn er drie klassen, `DefaultApi`, `PublicApi`, en `ModerationApi`. De `DefaultApi` bevat methoden die uw API-sleutel vereisen, en `PublicApi` bevat API-aanroepen die rechtstreeks vanuit een browser/mobiel apparaat/etc kunnen worden gedaan zonder authenticatie. De `ModerationApi` bevat de methoden die het moderator-dashboard aandrijven.

De `ModerationApi` bestrijkt commentaarmoderatie (lijst, aantal, zoeken, logs, export), moderatieacties (verwijderen/terugzetten, markeren, instellen van review/spam/goedkeuringsstatus, stemmen, opnieuw openen/sluiten van discussiedraad),
banbeheer (verbod bij een reactie, ongedaan maken, voor-ban samenvattingen, ban-status/voorkeuren, aantallen gebande gebruikers), en badges & vertrouwen (toekennen/verwijderen van badge, handmatige badges, ophalen/instellen van vertrouwensfactor, intern gebruikersprofiel).
Elke `ModerationApi`-methode accepteert een `sso`-parameter zodat het verzoek namens een via SSO geauthenticeerde moderator kan worden gedaan.