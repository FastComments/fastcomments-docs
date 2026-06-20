### PyPI

```bash
pip install fastcomments
```

### Zawartość biblioteki

Ta biblioteka zawiera dwa moduły: wygenerowanego klienta API oraz główną bibliotekę Python, która zawiera ręcznie napisane narzędzia ułatwiające pracę z API, w tym obsługę SSO.

- [Dokumentacja biblioteki klienta API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacja biblioteki głównej, w tym przykłady SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Publiczne vs zabezpieczone API

Dla klienta API dostępne są trzy klasy: `DefaultApi`, `PublicApi` i `ModerationApi`. Klasa `DefaultApi` zawiera metody wymagające Twojego klucza API, natomiast `PublicApi` zawiera metody, które można wywoływać bezpośrednio z przeglądarki/urządzenia mobilnego itp. bez uwierzytelniania. Klasa `ModerationApi` zasila panel moderatora i zawiera metody do moderowania komentarzy (lista, zliczanie, wyszukiwanie, logi, eksport), akcji moderacyjnych (usuń/przywróć, oznacz, ustaw status do przeglądu/spam/zatwierdzenie, głosy, ponowne otwarcie/zamknięcie wątku), banów (zablokuj od komentowania, cofnij, podsumowania przed banem, status/preferencje bana, liczby zbanowanych użytkowników) oraz odznak i zaufania (przyznaj/usuń odznakę, odznaki ręczne, pobierz/ustaw współczynnik zaufania, wewnętrzny profil użytkownika). Każda metoda `ModerationApi` akceptuje parametr `sso`, dzięki czemu może być wywołana w imieniu moderatora uwierzytelnionego przez SSO.