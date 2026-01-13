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

Deze bibliotheek bevat de gegenereerde API-client en de SSO-hulpprogramma's om het werken met de API makkelijker te maken.

- [Documentatie van de API-clientbibliotheek](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Openbare vs beveiligde API's

Voor de API-client zijn er twee klassen, `DefaultApi` en `PublicApi`. De `DefaultApi` bevat methoden die uw API-sleutel vereisen, en `PublicApi` bevat API-aanroepen die rechtstreeks vanuit een browser/mobiel apparaat/etc. kunnen worden gedaan zonder authenticatie.