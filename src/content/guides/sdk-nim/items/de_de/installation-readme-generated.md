### Verwendung von Nimble

```bash
nimble install fastcomments
```

### Aus dem Quellcode bauen

```bash
nimble build
```

### Bibliotheksinhalt

Diese Bibliothek enthält den generierten API‑Client und die SSO‑Dienstprogramme, um die Arbeit mit der API zu erleichtern.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Öffentliche vs gesicherte APIs

Für den API‑Client gibt es drei API‑Module, `api_default`, `api_public` und `api_moderation`. Das `api_default` enthält Methoden, die Ihren API‑Schlüssel benötigen, und `api_public` enthält API‑Aufrufe, die direkt von einem Browser/Mobilgerät/etc. ohne Authentifizierung durchgeführt werden können. Das `api_moderation`‑Modul enthält Methoden für das Moderator‑Dashboard.

Das `api_moderation`‑Modul bietet eine umfangreiche Suite von Live‑ und schnellen Moderations‑APIs. Jede `api_moderation`‑Methode akzeptiert einen `sso`‑Parameter und kann über SSO oder ein FastComments.com‑Session‑Cookie authentifiziert werden.