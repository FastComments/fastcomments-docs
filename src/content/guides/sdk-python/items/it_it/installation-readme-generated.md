### Installa da GitHub

Installa direttamente da un tag di rilascio (consigliato, completamente riproducibile):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Fissa il tag anziché un branch così le build sono deterministiche. Lo stesso formato funziona in `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Ogni [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) ha anche un wheel compilato allegato se preferisci installare direttamente un artefatto binario.

### Contenuto della Libreria

Questa libreria contiene due moduli: il client API generato e la libreria core Python che contiene utility scritte a mano per facilitare l'uso dell'API, inclusa il supporto SSO.

- [Documentazione della Libreria Client API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentazione della Libreria Core, Inclusi Esempi SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API Pubbliche vs Sicure

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi` e `ModerationApi`. La `DefaultApi` contiene metodi che richiedono la tua chiave API, e la `PublicApi` contiene metodi che possono essere chiamati direttamente da un browser/dispositivo mobile/ecc. senza autenticazione. La `ModerationApi` offre una vasta suite di API di moderazione in tempo reale e veloci. Ogni metodo della `ModerationApi` accetta un parametro `sso` e può autenticarsi tramite SSO o un cookie di sessione FastComments.com.