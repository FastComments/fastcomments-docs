### Abhängigkeiten installieren

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Aus dem Quellcode bauen

```bash
mkdir build
cd build
cmake ..
make
```

### Installation

```bash
sudo make install
```

### Bibliotheksinhalt

Diese Bibliothek enthält den generierten API-Client und die SSO-Dienstprogramme, um die Arbeit mit der API zu erleichtern.

- [API-Client-Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Öffentliche vs gesicherte APIs

Für den API-Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel erfordern, und die `PublicApi` enthält Methoden, die direkt von einem Browser/Mobilgerät usw. ohne Authentifizierung aufgerufen werden können. Die `ModerationApi` bietet eine umfangreiche Palette von Live- und schnellen Moderations-APIs. Jede `ModerationApi`-Methode akzeptiert einen `sso`-Parameter und kann sich über SSO oder ein FastComments.com‑Sitzungs‑Cookie authentifizieren.