### Installation von GitHub

Installieren Sie direkt von einem Release‑Tag (empfohlen, vollständig reproduzierbar):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Pinnen Sie das Tag anstatt eines Branches, damit Builds deterministisch sind. Die gleiche Form funktioniert in `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Jeder getaggte [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) enthält ebenfalls ein gebautes Wheel, falls Sie ein binäres Artefakt direkt installieren möchten.

### Bibliotheksinhalt

Diese Bibliothek enthält zwei Module: den generierten API‑Client und die Kern‑Python‑Bibliothek, die handgeschriebene Hilfsprogramme enthält, um die Arbeit mit der API zu vereinfachen, einschließlich SSO‑Unterstützung.

- [API‑Client‑Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Kernbibliotheksdokumentation, einschließlich SSO‑Beispielen](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Öffentliche vs gesicherte APIs

Für den API‑Client gibt es drei Klassen: `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API‑Schlüssel erfordern, und `PublicApi` enthält Methoden, die direkt von einem Browser/Mobilgerät usw. ohne Authentifizierung aufgerufen werden können. Die `ModerationApi` bietet eine umfangreiche Palette von Live‑ und schnellen Moderations‑APIs. Jede `ModerationApi`‑Methode akzeptiert einen `sso`‑Parameter und kann sich über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.