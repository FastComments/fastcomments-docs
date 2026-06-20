---
The FastComments Rust SDK składa się z kilku modułów:

- **Client Module** - API client for FastComments REST APIs
  - Pełne definicje typów dla wszystkich modeli API
  - Trzy klienty API obejmujące wszystkie metody FastComments:
    - `default_api` (**DefaultApi**) - metody uwierzytelniane kluczem API do użytku po stronie serwera
    - `public_api` (**PublicApi**) - publiczne metody bez klucza API bezpieczne do wywoływania z przeglądarek i aplikacji mobilnych
    - `moderation_api` (**ModerationApi**) - metody obsługujące panel moderatora, w tym moderację komentarzy (listowanie, zliczanie, wyszukiwanie, logi, eksport), działania moderacyjne (usuń/przywróć, oznacz, ustaw status przeglądu/spamu/akceptacji, głosy, ponowne otwarcie/zamknięcie wątku), blokady (ban z komentarza, cofnięcie, podsumowania przed zbanowaniem, stan/preferencje bana, liczba zbanowanych użytkowników) oraz odznaki i zaufanie (przyznawanie/usuwanie odznak, odznaki ręczne, pobierz/ustaw współczynnik zaufania, wewnętrzny profil użytkownika). Każda metoda moderacji akceptuje parametr `sso`, dzięki czemu wywołanie może być wykonane w imieniu moderatora uwierzytelnionego przez SSO.
  - Pełne wsparcie async/await z tokio
  - Zobacz [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) po szczegółową dokumentację API

- **SSO Module** - Narzędzia Single Sign-On po stronie serwera
  - Bezpieczne generowanie tokenów do uwierzytelniania użytkowników
  - Obsługa zarówno prostego, jak i bezpiecznego trybu SSO
  - Podpisywanie tokenów oparte na HMAC-SHA256

- **Core Types** - Wspólne definicje typów i narzędzia
  - Modele komentarzy i struktury metadanych
  - Konfiguracje użytkowników i tenantów
  - Funkcje pomocnicze dla typowych operacji
---