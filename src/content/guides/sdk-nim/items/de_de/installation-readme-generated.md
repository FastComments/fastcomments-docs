### Verwendung von Nimble

```bash
nimble install fastcomments
```

### Aus dem Quellcode bauen

```bash
nimble build
```

### Inhalt der Bibliothek

Diese Bibliothek enthält den generierten API-Client und die SSO-Dienstprogramme, um die Arbeit mit der API zu erleichtern.

- [Dokumentation der API-Client-Bibliothek](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es zwei API-Module, `api_default` und `api_public`. Das `api_default` enthält Methoden, die Ihren API-Schlüssel erfordern, und `api_public` enthält API-Aufrufe, die direkt aus einem Browser/Mobilgerät/etc. ohne Authentifizierung vorgenommen werden können.