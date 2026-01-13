### Utilizzo di Nimble

```bash
nimble install fastcomments
```

### Compilazione dal Sorgente

```bash
nimble build
```

### Contenuto della libreria

Questa libreria contiene il client API generato e le utilità SSO per facilitare l'interazione con l'API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### API pubbliche vs protette

Per il client API, ci sono due moduli API, `api_default` e `api_public`. `api_default` contiene metodi che richiedono la tua API key, e `api_public` contiene chiamate API
che possono essere effettuate direttamente da un browser/dispositivo mobile/etc. senza autenticazione.