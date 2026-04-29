A **test run** (zwany też **replay**) uruchamia agenta na wybranym oknie historycznych komentarzy **bez podejmowania rzeczywistych działań**. To najszybszy sposób na podgląd zachowania agenta przed uruchomieniem na żywo.

Dostępne z listy agentów poprzez przycisk **Test run** w wierszu każdego agenta.

### Co robi

Platforma:

1. Wybiera próbkę historycznych komentarzy pasujących do zakresu agenta, w oknie które wybierzesz.
2. Dla każdego komentarza uruchamia agenta end-to-end tak, jakby komentarz został właśnie opublikowany — ten sam kontekst, ten sam wywołanie LLM, ten sam wybór narzędzi, te same uzasadnienia i oceny pewności.
3. Rejestruje każde uruchomienie jako dry-run, oznaczając je tak, by pozostało pogrupowane z replayem, z którego pochodzi, i wykluczone z widoków uruchomień na żywo.
4. **Porównuje** werdykt agenta z tym, co faktycznie wydarzyło się z komentarzem — czy później został zatwierdzony, oznaczony jako spam, usunięty, zablokowany przez silnik antyspamowy itd.

Wynikiem jest różnica per komentarz: "Agent w replayu oznaczyłby to jako spam, ale komentarz jest obecnie zatwierdzony i czysty."

### Konfiguracja

Strona test-run ma jedno pole wejściowe:

- **Days of historical comments to evaluate** - pole numeryczne `days` między 1 a 90. Starsze komentarze nie są uprawnione.

Rozmiar próbki i twardy limit **nie są ujawniane w UI** - oba są domyślnymi ustawieniami po stronie serwera stosowanymi per plan. Strona pokazuje pola informacyjne:

- **Matching comments in window** - ile komentarzy byłoby branych pod uwagę.
- **Up to N comments from this window will be processed** - efektywny rozmiar próbki uwzględniający twardy limit po stronie serwera.
- **Estimated cost** - w walucie Twojego tenant'a.

### Limit częstotliwości

Każdy użytkownik ma ograniczenie do **10 test runów na 24 godziny** (rate-limited via key `replay-create:${requestedBy}`). Przycisk pokazuje dymek pomocy, gdy osiągniesz limit ("You've reached 10 test runs in the last 24 hours.").

### Współbieżność

Na jednego agenta może być aktywny tylko jeden replay naraz. Rozpoczęcie drugiego replaya, gdy jeden jest w toku, przekierowuje Cię do replaya w toku.

### Odczytywanie wyników

Gdy replay się zakończy, strona wyników pokazuje zakładki:

- **Deltas** (domyślnie aktywna) - werdykt agenta w replayu różni się od rzeczywistości. (Najciekawsze - "agent oznaczyłby ten komentarz jako spam, ale komentarz został zatwierdzony i jest w porządku".)
- **Matches** - werdykt agenta w replayu zgadza się z tym, co faktycznie się wydarzyło. (Poucające - agent zgadza się z rzeczywistością.)
- **No action** - agent w replayu zdecydował się nic nie robić. (Czasem poprawna odpowiedź; czasem agent coś przeoczył.)
- **All** - każdy wynik niezależnie od klasyfikacji.

Dla każdego komentarza w dowolnej zakładce:

- **Prior outcome** - klasyfikacja tego, co faktycznie się wydarzyło: **POZYTYWNY**, **NEGATYWNY**, lub **NIEOKREŚLONY**, z **Dowodami** ("Komentarz oznaczony jako usunięty dnia {date}", "Silnik: bayes" i tak dalej).
- **Replay agent would** - akcja wybrana przez agenta.
- **Why** - uzasadnienie.
- **Confidence** - wyświetlana jako procent.

### Dlaczego replay wymusza dry-run

Replay przeciwko komentarzowi, który został usunięty cztery miesiące temu, nie powinien go retroaktywnie usuwać — jest już usunięty. Replay przeciwko komentarzowi, który agent teraz chciałby zatwierdzić, nie powinien zmieniać aktualnego stanu komentarza. Replay jest narzędziem podglądu. Wymuszenie dry-run to to, co czyni bezpiecznym uruchamianie replaya na dowolnym oknie historii.

### Powtarzalność

Replaye zamrażają konfigurację agenta w momencie rozpoczęcia replaya. Późniejsze edycje agenta nie zmieniają wyników replaya — strona wyników pozostaje stabilnym zapisem tego, co ta wersja agenta by zrobiła.

### Kiedy budżety zatrzymują replay

Replaye podlegają:

- Własnemu **twardemu limitowi** (ustawanemu w formularzu replaya).
- Dziennym i miesięcznym **limitom budżetowym** agenta.
- Dziennym i miesięcznym **limitom budżetowym** tenant'a.

Pierwszy osiągnięty limit przerywa replay z określonym kodem błędu. Jakiekolwiek wyniki per-komentarz wygenerowane przed przerwaniem są zachowywane w [Historia uruchomień](#run-history).

### Jak działają replaye

Replaye działają w tle, nie synchronicznie. Po kliknięciu "Start test run", replay jest umieszczany w kolejce i pobiera go worker. Długi replay może trwać kilka minut. Strona wyników odpyta i pokaże postęp (liczba przetworzonych, wydatki do tej pory) w trakcie.

Jeśli worker padnie w trakcie replaya, platforma automatycznie ponownie umieści replay w kolejce, dzięki czemu wznowi się przy następnym przebiegu. Krótkie zakłócenie nigdy nie porzuca replaya.

### Czego replay nie robi

- **Nie respektuje [trigger delays](#trigger-deferred-delay).** Replaye uruchamiają się natychmiast, nie za 30 minut.
- **Nie zapisuje do pamięci.** Agenci w replayu nie zapisują notatek do pamięci, nawet jeśli ich logika normalnie by to robiła.
- **Nie wysyła webhooków.** Wyzwalacze wygenerowane przez replay nie generują zdarzeń webhook `trigger.succeeded` / `trigger.failed`.
- **Nie wyklucza już-replayowanych komentarzy.** Uruchomienie drugiego replaya na tym samym oknie obejmuje te same komentarze.

### Zobacz także

- [Refining Prompts](#refining-prompts) - workflow iteracyjnego edytowania, który dobrze współgra z replayami.
- [Dry-Run Mode](#dry-run-mode) - ta sama idea, na ruchu na żywo.