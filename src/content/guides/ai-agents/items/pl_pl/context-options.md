Sekcja **Kontekst** na formularzu edycji kontroluje, ile informacji agent otrzymuje przy każdym uruchomieniu. Więcej kontekstu daje lepsze decyzje, ale zwiększa koszt tokenów na uruchomienie, więc chcesz mieć tylko to, czego agent faktycznie potrzebuje.

### Co jest zawsze uwzględnione

Nawet jeśli wszystkie pola wyboru są odznaczone, wiadomość kontekstowa agenta zawiera:

- Typ zdarzenia wyzwalającego (np. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- Adres URL strony i identyfikator URL (gdy znany).
- Komentarz, który wywołał uruchomienie, jeśli taki istnieje — ID, identyfikator użytkownika autora, wyświetlana nazwa autora, tekst komentarza, liczby głosów, liczba zgłoszeń, flagi spam/zaakceptowano/przejrzano, ID rodzica. Email autora **nigdy** nie jest wysyłany do dostawcy LLM (minimalizacja PII).
- Poprzedni tekst komentarza dla wyzwalaczy `COMMENT_EDIT` (aby agent mógł porównać przed/po).
- Kierunek głosu dla wyzwalaczy `COMMENT_VOTE_THRESHOLD`.
- ID użytkownika wyzwalającego oraz ID odznaki (dla wyzwalaczy związanych z odznakami moderatora).

Wszystkie nienadane (niezaufane) teksty — treści komentarzy, nazwy autorów, tytuły stron, sam dokument wytycznych — są **odgradzane** w wiadomości kontekstowej znacznikami takimi jak `<<<COMMENT_TEXT>>> ... <<<END>>>`. Systemowy prompt platformy instruuje model, aby nigdy nie wykonywał poleceń znajdujących się wewnątrz tych ogrodzeń. To jest obrona platformy przed wstrzyknięciem prompta; nie musisz tego powtarzać w swoim promptcie.

### Te trzy pola wyboru

#### Uwzględnij komentarz nadrzędny i wcześniejsze odpowiedzi w tym samym wątku

Dodaje:
- **komentarz nadrzędny** - ID, autor, tekst.
- **odpowiedzi rodzeństwa** - wcześniejsze odpowiedzi do tego samego rodzica w tym samym wątku.

Przydatne dla: każdego agenta, który odpowiada na komentarz w kontekście (powitalni witacze, podsumowywacze wątków, moderatorzy czytający odpowiedzi w konwersacjach).

Koszt: mały do średniego. Ograniczony liczbą odpowiedzi w danym wątku.

#### Uwzględnij współczynnik zaufania komentującego, wiek konta, historię banów i ostatnie komentarze

Dodaje blok **AUTHOR_HISTORY**:

- Wiek konta w dniach od rejestracji.
- Współczynnik zaufania (0–100) - wynik FastComments podsumowujący, jak bardzo użytkownik jest zaufany na tej stronie. Zobacz stronę [Wykrywanie spamu](/guide-moderation.html#spam-detection) w przewodniku moderacji.
- Liczba wcześniejszych banów.
- Łączna liczba komentarzy na tej stronie.
- Liczba duplikatów treści - jeśli użytkownik opublikował identyczny tekst niedawno (sygnał antyspamowy).
- Sygnał cross-account z tego samego IP - liczba komentarzy z tego samego IP pod innymi kontami (sygnał kont alternatywnych). Sam hash IP nigdy nie jest wysyłany do LLM.
- Ostatnie komentarze - do 5 najnowszych komentarzy użytkownika, każdy obcięty do 300 znaków, oddzielone znacznikami jako tekst nieufny.

Przydatne dla: każdego agenta moderującego. Bez tego model będzie blokował nowe konta i długoletnich użytkowników działających w dobrej wierze, którzy przejawiają podobne zachowanie.

Koszt: średni. Najwięcej tokenów dodają ostatnie komentarze.

#### Uwzględnij tytuł strony, podtytuł, opis i meta tagi

Dodaje blok **PAGE_CONTEXT** — tytuł, podtytuł, opis oraz wszelkie meta tagi, które FastComments przechwycił dla strony.

Przydatne dla: botów powitalnych i podsumowywaczy wątków, gdzie znajomość tematu strony znacząco poprawia jakość wyników.

Koszt: mały.

### Wytyczne społeczności

Czwarte pole, **Wytyczne społeczności**, to pole polityki w formie wolnego tekstu dołączane do wiadomości kontekstowej w roli użytkownika przy każdym uruchomieniu, oddzielone znacznikami jako tekst nieufny w taki sam sposób jak treści komentarzy i inne treści dostarczone przez użytkowników. Agent traktuje je jako tekst polityki, ale platforma nie traktuje go jako instrukcji systemowej. Zobacz [Wytyczne społeczności](#community-guidelines), co w nim umieścić.

### Dodawanie kontekstu selektywnie

Te pola wyboru dotyczą poszczególnych agentów, a nie ustawień globalnych. Typowy wzorzec:

- Bot powitalny: kontekst strony **włączony**, kontekst wątku **wyłączony**, historia użytkownika **wyłączona**.
- Moderator: kontekst wątku **wyłączony**, historia użytkownika **włączona**, kontekst strony **wyłączony**.
- Podsumowywacz wątków: kontekst wątku **włączony**, kontekst strony **włączony**, historia użytkownika **wyłączona**.

Sięgaj po minimalny kontekst, jakiego agent potrzebuje, aby działać poprawnie w wywołaniach, które faktycznie wykonuje — dodatkowy kontekst generuje koszty tokenów przy każdym uruchomieniu, nawet jeśli agent go nie używa.