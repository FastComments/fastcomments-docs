### Instalacja zależności

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Budowanie ze źródła

```bash
mkdir build
cd build
cmake ..
make
```

### Instalacja

```bash
sudo make install
```

### Zawartość biblioteki

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO, które ułatwiają pracę z API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Publiczne vs Zabezpieczone API

Dla klienta API istnieją trzy klasy: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` zawiera metody, które wymagają klucza API, a `PublicApi` zawiera
metody, które mogą być wywoływane bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelnienia. `ModerationApi` oferuje rozbudowany zestaw szybkich i bieżących interfejsów moderacji. Każda metoda `ModerationApi` przyjmuje parametr `sso` i może uwierzytelnić się za pomocą SSO lub pliku cookie sesji FastComments.com.