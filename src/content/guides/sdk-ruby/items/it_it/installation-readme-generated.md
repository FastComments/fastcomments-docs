Aggiungi questa riga al Gemfile della tua applicazione:

```ruby
gem 'fastcomments'
```

E poi esegui:

```bash
bundle install
```

Oppure installalo tu stesso con:

```bash
gem install fastcomments
```

### Contenuto della libreria

Questa libreria contiene il client API generato e le utility SSO per semplificare l'uso dell'API.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### API Pubbliche vs Protette

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi`, e `ModerationApi`. La `DefaultApi` contiene i metodi che richiedono la tua API key, e `PublicApi` contiene chiamate api
che possono essere effettuate direttamente da un browser/dispositivo mobile/etc senza autenticazione. La `ModerationApi` contiene i metodi che alimentano la dashboard del moderatore.

La `ModerationApi` copre la moderazione dei commenti (elenco, conteggio, ricerca, registri, esportazione), azioni di moderazione (rimuovi/ripristina, segnala, impostare stato revisione/spam/approvazione, voti, riaprire/chiudere thread),
ban (bannare da un commento, annulla, riepiloghi pre-ban, stato/preferenze del ban, conteggi utenti bannati), e badge & fiducia (assegnare/rimuovere badge, badge manuali, ottenere/impostare fattore di fiducia, profilo interno utente).
Ogni metodo di `ModerationApi` accetta un parametro `sso` in modo che la richiesta possa essere effettuata per conto di un moderatore autenticato via SSO.