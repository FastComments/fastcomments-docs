### Jak działa zaznaczanie tekstu

Gdy użytkownicy zaznaczają tekst w kontenerze Collab Chat, widget przechwytuje to zaznaczenie i umożliwia rozpoczęcie dyskusji. Zaznaczenie może obejmować pojedyncze słowo lub kilka akapitów rozciągających się przez różne elementy.

### Opóźnienie wyświetlania

Na komputerach stacjonarnych występuje opóźnienie 3,5 sekundy między momentem zaznaczenia tekstu a pojawieniem się wezwania do dyskusji. Zapobiega to migotaniu interfejsu, gdy użytkownicy jedynie zaznaczają tekst, aby go skopiować lub przeczytać. Na urządzeniach mobilnych wezwanie pojawia się natychmiast, ponieważ zaznaczanie tekstu na ekranach dotykowych jest zwykle bardziej zamierzone.

### Unikalne identyfikatory konwersacji

Każda konwersacja otrzymuje unikalny `urlId`, który łączy URL strony, ścieżkę elementu DOM oraz zserializowany zakres tekstu. Zapewnia to, że każde zaznaczenie tekstu tworzy odrębną konwersację, którą można później ponownie odnaleźć.

Jeśli w konfiguracji podasz niestandardowy `urlId`, zostanie on połączony ze ścieżką elementu i zakresem tekstu, aby utworzyć końcowy identyfikator.

### Wyróżnienia wizualne

Gdy dla konkretnego zaznaczenia tekstu istnieje dyskusja, tekst ten otrzymuje wyróżnienie wizualne. Wyróżnienie jest realizowane przy użyciu kolorów tła i pojawia się przy najechaniu kursorem lub gdy powiązane okno czatu jest otwarte.

System wyróżnień działa poprzez opakowanie zaznaczonego tekstu w specjalny element, który można wystylizować. Takie podejście zapewnia, że wyróżnienia pozostają dokładne nawet wtedy, gdy struktura HTML jest skomplikowana.

### Pozycjonowanie okna czatu

Gdy użytkownik kliknie wyróżnienie lub utworzy nowe adnotacje, w pobliżu zaznaczonego tekstu pojawia się okno czatu. Widget automatycznie oblicza najlepszą pozycję tego okna na podstawie dostępnej przestrzeni w widoku.

System pozycjonowania używa klas CSS takich jak `to-right`, `to-left`, `to-top` i `to-bottom`, aby wskazać, w którym kierunku okno czatu powinno się rozciągać od wyróżnienia. Na urządzeniach mobilnych (ekrany o szerokości poniżej 768px) okno czatu zawsze pojawia się w trybie pełnoekranowym dla lepszej użyteczności.

### Wymiary okna czatu

Okna czatu mają szerokość 410px na komputerach stacjonarnych, z odstępem 20px i 16px wizualną strzałką wskazującą na wyróżniony tekst. Te wymiary są stałe, aby zapewnić spójne zachowanie, ale wygląd można dostosować za pomocą CSS.

### Zaznaczenia obejmujące wiele elementów

Użytkownicy mogą zaznaczać tekst rozciągający się przez wiele elementów HTML, na przykład od środka jednego akapitu do początku innego. System serializacji zakresu obsługuje to poprawnie i wyróżni cały zaznaczony tekst nawet na granicach elementów.

### Kompatybilność z przeglądarkami

System zaznaczania tekstu używa standardowego API `window.getSelection()`, które jest obsługiwane we wszystkich nowoczesnych przeglądarkach. Dla starszych wersji Internet Explorera stosowany jest fallback do `document.selection` w celu zachowania kompatybilności.

### Trwałość zaznaczeń

Po utworzeniu konwersacji dla zaznaczenia tekstu, ta adnotacja pozostaje nawet po odświeżeniu strony. Zserializowany zakres i ścieżka DOM pozwalają widgetowi przywrócić wyróżnienia w dokładnie tym samym miejscu, gdy użytkownicy wrócą na stronę.

Działa to niezawodnie, o ile zawartość strony pozostaje stabilna. Jeśli zmienisz treść tekstu lub przebudujesz strukturę HTML, istniejące adnotacje mogą przestać prawidłowo odpowiadać tekstowi. Z tego powodu najlepiej unikać większych zmian zawartości na stronach z aktywnymi adnotacjami lub rozważyć migrację adnotacji, gdy zmiany treści są niezbędne.