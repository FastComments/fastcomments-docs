### Installeer afhankelijkheden

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Bouwen vanuit bron

```bash
mkdir build
cd build
cmake ..
make
```

### Installeren

```bash
sudo make install
```

### Bibliotheekinhoud

Deze bibliotheek bevat de gegenereerde API-client en de SSO-hulpmiddelen om het werken met de API gemakkelijker te maken.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Publieke versus beveiligde API's

Voor de API-client zijn er drie klassen, `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die uw API-sleutel vereisen, en `PublicApi` bevat  
methoden die rechtstreeks vanaf een browser/mobiel apparaat/etc. kunnen worden aangeroepen zonder authenticatie. De `ModerationApi` biedt een uitgebreide reeks van live en snelle moderatie-API's. Elke `ModerationApi`-methode accepteert een `sso`-parameter en kan authenticeren via SSO of een FastComments.com sessiecookie.