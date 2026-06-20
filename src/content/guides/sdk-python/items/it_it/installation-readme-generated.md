### PyPI

```bash
pip install fastcomments
```

### Contenuti della libreria

Questa libreria contiene due moduli: il client API generato e la libreria core Python che include utility scritte a mano per rendere più semplice il lavoro con l'API, incluso il supporto SSO.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentazione della libreria core, inclusi esempi SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API pubbliche vs protette

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi`, e `ModerationApi`. La `DefaultApi` contiene metodi che richiedono la tua API key, e `PublicApi` contiene metodi che possono essere eseguiti direttamente da un browser/dispositivo mobile/etc. senza autenticazione. La `ModerationApi` alimenta la dashboard dei moderatori e contiene metodi per la moderazione dei commenti (elenco, conteggio, ricerca, log, esportazione), azioni di moderazione (rimuovi/ripristina, segnala, imposta stato revisione/spam/approvazione, voti, riapri/chiudi thread), ban (vietare di commentare, annulla, riepiloghi pre-ban, stato/preferenze ban, conteggi utenti bannati), e badge & trust (assegna/rimuovi badge, badge manuali, ottieni/imposta fattore di fiducia, profilo interno utente). Ogni metodo di `ModerationApi` accetta un parametro `sso` in modo che possa essere chiamato per conto di un moderatore autenticato via SSO.