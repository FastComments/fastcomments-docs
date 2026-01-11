---
### PyPI

```bash
pip install fastcomments
```

### Bibliotheksinhalte

Diese Bibliothek enthält zwei Module: den generierten API-Client und die Kern-Python-Bibliothek, die handgeschriebene Hilfsfunktionen enthält, um die Arbeit mit der API zu erleichtern, einschließlich SSO-Unterstützung.

- [API-Client-Bibliothek Dokumentation](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Kernbibliothek Dokumentation, einschließlich SSO-Beispiele](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es zwei Klassen, `DefaultApi` und `PublicApi`. Die `DefaultApi` enthält Methoden, die deinen API-Schlüssel benötigen, und `PublicApi` enthält API-Aufrufe, die direkt aus einem Browser/Mobilgerät/etc. ohne Authentifizierung durchgeführt werden können.
---