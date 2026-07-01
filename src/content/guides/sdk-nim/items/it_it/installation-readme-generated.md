---
### Utilizzare Nimble

```bash
nimble install fastcomments
```

### Compilare dal codice sorgente

```bash
nimble build
```

### Contenuto della libreria

Questa libreria contiene il client API generato e le utility SSO per semplificare l'utilizzo dell'API.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### API pubbliche vs API sicure

Per il client API, esistono tre moduli API, `api_default`, `api_public` e `api_moderation`. Il modulo `api_default` contiene metodi che richiedono la tua chiave API, e `api_public` contiene chiamate API che possono essere effettuate direttamente da un browser/dispositivo mobile/etc. senza autenticazione. Il modulo `api_moderation` contiene metodi per la dashboard del moderatore.

Il modulo `api_moderation` offre una suite completa di API di moderazione live e rapide. Ogni metodo `api_moderation` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione di FastComments.com.
---