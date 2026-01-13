### Afhankelijkheden installeren

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

### Inhoud van de bibliotheek

Deze bibliotheek bevat de gegenereerde API-client en de SSO-hulpprogramma's om het werken met de API te vergemakkelijken.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Publieke vs beveiligde API's

Voor de API-client zijn er twee klassen, `DefaultAPI` en `PublicAPI`. De `DefaultAPI` bevat methoden die uw API key vereisen, en `PublicAPI` bevat API-aanroepen
die rechtstreeks vanuit een browser/mobiel apparaat/etc. zonder authenticatie kunnen worden gedaan.