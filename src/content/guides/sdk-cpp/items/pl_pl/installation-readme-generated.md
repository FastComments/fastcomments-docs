### Zainstaluj zależności

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Kompilacja ze źródeł

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

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Publiczne vs zabezpieczone API

Klient API zawiera trzy klasy: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` zawiera metody wymagające klucza API, a `PublicApi` zawiera
metody, które można wywoływać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania. `ModerationApi` zawiera metody, które zasilają panel moderatora - listowanie,
zliczanie, wyszukiwanie, eksportowanie i pobieranie logów komentarzy, działania moderacyjne (usuwanie/przywracanie, oznaczanie, ustawianie statusu do przeglądu/spam/akceptacji, zmiana głosów, ponowne otwieranie/zamykanie wątków),
bany (zablokowanie względem komentarza, cofanie banów, podsumowania przed banem, status i preferencje banów, liczby zablokowanych użytkowników), oraz odznaki i zaufanie (przyznawanie/usuwanie odznak, odznaki ręczne, pobieranie/ustawianie współczynnika zaufania, wewnętrzny profil użytkownika). Każda metoda `ModerationApi` przyjmuje parametr `sso` więc wywołanie jest wykonywane w imieniu moderatora uwierzytelnionego przez SSO.