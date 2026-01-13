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

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es zwei Klassen, `DefaultAPI` und `PublicAPI`. Die `DefaultAPI` enthält Methoden, die Ihren API-Schlüssel benötigen, und `PublicAPI` enthält API-Aufrufe
die direkt von einem Browser/Mobilgerät/etc. ohne Authentifizierung ausgeführt werden können.