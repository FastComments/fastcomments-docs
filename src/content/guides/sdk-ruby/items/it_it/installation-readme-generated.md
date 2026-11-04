Aggiungi questa riga al Gemfile della tua applicazione:

```ruby
gem 'fastcomments'
```

Quindi esegui:

```bash
bundle install
```

Oppure installala tu stesso con:

```bash
gem install fastcomments
```

### Contenuto della Libreria

Questa libreria contiene il client API generato e le utilità SSO per facilitare l'uso dell'API.

- [Documentazione della Libreria Client API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API Pubbliche vs API Protette

Per il client API, esistono tre classi: `DefaultApi`, `PublicApi` e `ModerationApi`. La `DefaultApi` contiene i metodi che richiedono la tua API key, mentre la `PublicApi` contiene le chiamate API
che possono essere effettuate direttamente da un browser/dispositivo mobile/ecc senza autenticazione. La `ModerationApi` contiene i metodi che alimentano la dashboard del moderatore.

La `ModerationApi` fornisce un'ampia suite di API di moderazione live e veloci. Ogni metodo della `ModerationApi` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione di FastComments.com.