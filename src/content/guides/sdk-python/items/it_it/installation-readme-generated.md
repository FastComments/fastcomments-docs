### PyPI

```bash
pip install fastcomments
```

### Contenuti della libreria

Questa libreria contiene due moduli: il client API generato e la libreria core Python che contiene utility scritte a mano per rendere più semplice lavorare con l'API, incluso il supporto SSO.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentazione della libreria core, inclusi esempi SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API Pubbliche vs Protette

Per il client API, ci sono due classi, `DefaultApi` e `PublicApi`. La `DefaultApi` contiene metodi che richiedono la tua API key, e `PublicApi` contiene chiamate API che possono essere eseguite direttamente da un browser/dispositivo mobile/etc senza autenticazione.