Sekcja **Kontekst** w formularzu edycji kontroluje, ile informacji agent otrzymuje przy każdym uruchomieniu. Większy kontekst daje lepsze decyzje, ale zwiększa koszt tokenów na uruchomienie, więc chcesz przekazywać jedynie to, czego agent rzeczywiście potrzebuje.

### Co jest zawsze dołączone

Nawet jeśli wszystkie pola wyboru są odznaczone, wiadomość kontekstowa agenta zawiera:

- Typ zdarzenia wyzwalającego (**trigger event type**) (np. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- URL strony i ID URL (jeśli znane).
- Komentarz, który wywołał uruchomienie, jeśli taki istnieje - ID, ID autora użytkownika, wyświetlana nazwa autora, tekst komentarza, liczba głosów, liczba zgłoszeń, flagi spam/zaakceptowane/przejrzane, ID rodzica. Email autora **nigdy** nie jest wysyłany do dostawcy LLM (minimalizacja PII).
- Poprzedni tekst komentarza dla wyzwalaczy `COMMENT_EDIT` (aby agent mógł porównać przed/po).
- Kierunek głosu dla wyzwalaczy `COMMENT_VOTE_THRESHOLD`.
- ID użytkownika, który wywołał zdarzenie, oraz ID odznaki (dla wyzwalaczy odznak moderatora).
- Katalog odznak Twojego tenant'a (**badge catalog**) (nazwa, etykieta wyświetlana, opis), gdy agent ma pozwolenie na przyznawanie odznak, aby mógł wybrać odpowiednią bez konieczności wypisywania odznak w promptcie.

Wszystkie niedozwolone (untrusted) teksty — treści komentarzy, nazwy autorów, tytuły stron, sam dokument wytycznych — są **ogrodzone** w wiadomości kontekstowej znacznikami takimi jak `<<<COMMENT_TEXT>>> ... <<<END>>>`. Prompt systemowy platformy instruuje model, by nigdy nie wykonywał poleceń znajdujących się wewnątrz tych ogrodzeń. To jest mechanizm obrony przed wstrzykiwaniem promptów; nie musisz go powtarzać w swoim promptcie.

### Te trzy pola wyboru

#### Dołącz komentarz rodzica i wcześniejsze odpowiedzi w tym samym wątku

Dodaje:
- Komentarz **rodzica** - ID, autor, tekst.
- **Odpowiedzi rodzeństwa** - wcześniejsze odpowiedzi do tego samego rodzica w tym samym wątku.

Przydatne dla: każdego agenta, który odpowiada na komentarz w kontekście (powitania, podsumowywacze wątków, moderatorzy czytający odpowiedzi w rozmowach).

Koszt: mały do średniego. Ograniczony przez liczbę odpowiedzi rodzeństwa w danym wątku.

#### Dołącz współczynnik zaufania komentującego, wiek konta, historię banów i ostatnie komentarze

Dodaje blok **AUTHOR_HISTORY**:

- **Wiek konta w dniach** od rejestracji.
- **Wskaźnik zaufania (0-100)** - wynik FastComments, który podsumowuje, jak bardzo użytkownik jest zaufany na tej stronie. Zobacz stronę [Wykrywanie spamu](/guide-moderation.html#spam-detection) w przewodniku moderacji.
- **Liczba wcześniejszych banów.**
- **Łączna liczba komentarzy na tej stronie.**
- **Liczba duplikatów treści** - jeśli użytkownik niedawno zamieszczał identyczne teksty (sygnał antyspamowy).
- **Sygnał współdzielenia IP między kontami** - liczba komentarzy z tego samego IP pod innymi kontami (sygnał alt-kont). Sam hash IP nigdy nie jest wysyłany do LLM.
- **Ostatnie komentarze** - do 5 najnowszych komentarzy użytkownika, każdy przycięty do 300 znaków, ogrodzony jako niedozwolony tekst.

Przydatne dla: każdego agenta moderującego. Bez tego model blokuje nowe konta i długo działających, działających w dobrej wierze użytkowników w podobnych okolicznościach.

Koszt: średni. Najwięcej tokenów dodają ostatnie komentarze.

#### Dołącz tytuł strony, podtytuł, opis i meta tagi

Dodaje blok **PAGE_CONTEXT** - tytuł, podtytuł, opis oraz wszelkie meta tagi, które FastComments zebrał dla strony.

Przydatne dla: agentów witających i podsumowujących wątki, gdzie znajomość tematu strony znacznie poprawia jakość outputu.

Koszt: mały.

### Wytyczne społeczności

Czwarte pole, **Community guidelines**, to pole swobodnego tekstu zawierające politykę, dołączane do wiadomości kontekstowej w roli użytkownika przy każdym uruchomieniu, ogrodzone jako niedozwolony tekst w taki sam sposób, jak treści komentarzy i inne dane dostarczone przez użytkownika. Agent odczytuje je jako tekst polityki, ale platforma nie traktuje tego jako instrukcji systemowej. Zobacz [Community Guidelines](#community-guidelines) aby dowiedzieć się, co w nim umieścić.

### Dodawanie kontekstu selektywnie

Te pola wyboru dotyczą konkretnego agenta, a nie ustawień globalnych. Typowe wzorce:

- Powitanie użytkownika: kontekst strony **włączony**, kontekst wątku **wyłączony**, historia użytkownika **wyłączona**.
- Moderator: kontekst wątku **wyłączony**, historia użytkownika **włączona**, kontekst strony **wyłączony**.
- Podsumowywacz wątków: kontekst wątku **włączony**, kontekst strony **włączony**, historia użytkownika **wyłączona**.

Sięgnij po minimalny kontekst, którego agent potrzebuje, aby poprawnie wykonać wywołania, które faktycznie wykonuje — dodatkowy kontekst zwiększa koszty tokenów przy każdym uruchomieniu, nawet gdy agent go nie używa.