Podzbiór działań moderacyjnych można wykonać bezpośrednio z wątku komentarzy, bez konieczności przechodzenia do strony Moderacja komentarzy.

Po zalogowaniu kliknij przycisk edycji w prawym górnym rogu komentarza. Jako moderator powinieneś mieć następujące opcje:

- **Przypnij** ten komentarz
- **Usuń** ten komentarz
- **Usuń** ten komentarz + **Zbanuj użytkownika** (na stałe lub shadow, więcej szczegółów dalej)
- **Edytuj** ten komentarz
- **Zablokuj** lub **Odblokuj** ten komentarz (więcej szczegółów poniżej)
- Oznacz ten komentarz jako **Zatwierdzony** (pokaż) lub **Niezatwierdzony** (ukryj)
- Oznacz ten komentarz jako **Spam** lub **Nie spam**

### Zablokowanie komentarza

Zablokowanie pojedynczego komentarza uniemożliwia dodawanie do niego nowych odpowiedzi, a także zapobiega edycji lub usunięciu komentarza do czasu jego odblokowania. Dotyczy to wszystkich, włącznie z administratorami i moderatorami. Jeśli musisz edytować lub usunąć zablokowany komentarz, najpierw go odblokuj, dokonaj zmiany, a następnie ponownie zablokuj, jeśli to konieczne.

W prawym górnym rogu zablokowanego komentarza pojawia się ikona kłódki, aby czytelnicy mogli na pierwszy rzut oka zobaczyć, że wątek jest zamknięty. Pozycje menu Edytuj i Usuń są ukryte dla zablokowanych komentarzy zarówno w widżecie komentarzy, jak i w publicznym API (`PATCH` i `DELETE` zwracają `code: 'locked'`, jeśli zostaną wywołane wobec zablokowanego komentarza).

Istnieją dwa celowe wyjątki omijające blokadę, ponieważ w przeciwnym razie pozostawiłyby za sobą osierocone dane: kiedy użytkownik usuwa całe swoje konto (jego komentarze są usuwane niezależnie od stanu blokady), oraz kiedy moderator banuje użytkownika z opcją "usuń wszystkie komentarze tego użytkownika" (sprzątanie usuwa blokady).

### Zamykanie wątków komentarzy

Moderatorzy i administratorzy mogą zablokować, czyli zamknąć, wątki komentarzy, wybierając `Close Thread` w menu z trzema kropkami u góry obszaru komentarzy, jeśli są zalogowani. Później w dowolnym momencie mogą wybrać `Re-Open Thread`, aby ponownie otworzyć możliwość komentowania.

Zamknięcie wątku komentarzy uniemożliwia dodawanie nowych komentarzy, ale nadal pozwala na głosowanie i na usuwanie własnych komentarzy przez użytkowników, jeśli tego chcą.

Zamykanie i ponowne otwieranie wątków komentarzy wpływa natychmiast na wszystkich użytkowników oglądających wątek.

Możesz także oznaczyć wątek jako tylko do odczytu, co usuwa również opcje głosowania i usuwania, tworząc regułę dostosowania specjalnie dla tej strony.

### Aktualizacje na żywo

Wszystkie te działania natychmiast zaktualizują wątki komentarzy innych użytkowników bez konieczności odświeżania strony. Jednak działania moderatorów, takie jak ukrycie komentarza lub oznaczenie go jako spam, nie usuwają komentarza z ekranu **moderatora**, aby w razie potrzeby mógł szybko cofnąć akcję. Aby wskazać, że komentarz jest ukryty, zostanie on wyróżniony w porównaniu z innymi komentarzami (kolor wyróżnienia zależy od powodu usunięcia).

Na przykład, zakładając użytkowników `A (commenter)`, `B (Moderator 1)`, oraz `C (Moderator 2)`.

...i następujący scenariusz:

1. `User B (Moderator 1)` ukrywa komentarz.
2. Dla `User A (commenter)` ten komentarz jest natychmiast ukryty.
3. Dla `User C (Moderator 2)` ten komentarz jest natychmiast ukryty.
4. Dla użytkownika, który wykonał zmianę, `User B (Moderator 1)`, komentarz pozostaje na jego ekranie, ale jest wyróżniony jako usunięty. Mają oni możliwość cofnięcia swojej akcji, w takim przypadku inni użytkownicy ponownie zobaczą aktualizację na żywo.