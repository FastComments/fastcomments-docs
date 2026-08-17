### Installa da GitHub

Installa direttamente da un tag di rilascio (consigliato, completamente riproducibile):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Fissa il tag anziché un branch così le build sono deterministiche. La stessa forma funziona in `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Ogni [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) etichettato ha anche una wheel compilata allegata se preferisci installare direttamente un artefatto binario.

### Contenuto della Libreria

Questa libreria contiene due moduli: il client API generato e la libreria core Python che contiene utility scritte a mano per semplificare l'uso dell'API, incluso il supporto SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API Pubbliche vs Sicure

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi` e `ModerationApi`. Il `DefaultApi` contiene metodi che richiedono la tua chiave API, e `PublicApi` contiene metodi che possono essere chiamati direttamente da un browser/dispositivo mobile/etc senza autenticazione. Il `ModerationApi` fornisce una suite estesa di API di moderazione live e veloci. Ogni metodo `ModerationApi` accetta un parametro `sso` e può autenticarsi via SSO o tramite un cookie di sessione FastComments.com.