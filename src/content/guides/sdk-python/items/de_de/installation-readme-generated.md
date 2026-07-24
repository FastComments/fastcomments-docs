### Installation von GitHub

Direkt von einem Release-Tag installieren (empfohlen, vollständig reproduzierbar):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Den Tag anstelle eines Branches festlegen, damit Builds deterministisch sind. Die gleiche Form funktioniert in `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Jeder getaggte [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) hat ebenfalls ein gebautes Wheel angehängt, falls Sie ein binäres Artefakt direkt installieren möchten.

### Bibliotheksinhalt

Diese Bibliothek enthält zwei Module: den generierten API-API‑Client und die Kern‑Python‑Bibliothek, die handgeschriebene Hilfsprogramme enthält, um die Arbeit mit der API zu erleichtern, einschließlich SSO‑Unterstützung.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Öffentliche vs gesicherte APIs

Für den API-Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API‑Schlüssel benötigen, und die `PublicApi` enthält Methoden, die direkt von einem Browser/Mobilgerät/etc. ohne Authentifizierung aufgerufen werden können. Die `ModerationApi` bietet eine umfangreiche Suite von Live‑ und schnellen Moderations‑APIs. Jede `ModerationApi`‑Methode akzeptiert einen `sso`‑Parameter und kann sich über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.