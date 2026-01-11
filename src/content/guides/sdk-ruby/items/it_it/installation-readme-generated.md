Aggiungi questa riga al Gemfile della tua applicazione:

```ruby
gem 'fastcomments'
```

E poi esegui:

```bash
bundle install
```

Oppure installalo manualmente con:

```bash
gem install fastcomments
```

### Contenuti della libreria

Questa libreria contiene il client API generato e le utilità SSO per semplificare il lavoro con l'API.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API pubbliche vs API protette

Per il client API, ci sono due classi, `DefaultApi` e `PublicApi`. La `DefaultApi` contiene metodi che richiedono la tua chiave API, e la `PublicApi` contiene chiamate API che possono essere effettuate direttamente da un browser/dispositivo mobile/etc. senza autenticazione.