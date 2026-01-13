### Instalacja zależności

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Budowanie ze źródeł

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

Ta biblioteka zawiera wygenerowanego klienta API oraz narzędzia SSO, aby ułatwić pracę z API.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Publiczne vs. zabezpieczone API

Dla klienta API istnieją dwie klasy, `DefaultAPI` i `PublicAPI`. `DefaultAPI` zawiera metody, które wymagają twojego klucza API, a `PublicAPI` zawiera wywołania API, które można wykonywać bezpośrednio z przeglądarki/urządzenia mobilnego/itd. bez uwierzytelniania.