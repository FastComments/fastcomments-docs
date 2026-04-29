Agentowe **narzędzia** to działania, które może wykonać. Formularz edycji agenta ma sekcję **Allowed tool calls**, w której zaznaczasz narzędzia, z których agent może korzystać, oraz sekcję **Approvals**, w której zaznaczasz działania wymagające zatwierdzenia przez człowieka zanim wejdą w życie.

Dostępne są trzy poziomy dla każdego narzędzia:

- **Niedozwolone** - agent nie może go zobaczyć ani używać.
- **Dozwolone, bez zatwierdzenia** - agent używa go bezpośrednio. Zapisane w historii uruchomień.
- **Dozwolone, z zatwierdzeniem** - wywołanie agenta jest umieszczane w kolejce do przeglądu przez człowieka i uruchamiane tylko po jego zatwierdzeniu.

Niedozwolone narzędzia są niesłyszalne: agent nie może o nie prosić, a platforma odmawia ich wprost. Narzędzia z bramką zatwierdzenia zawsze przechodzą przez [skrzynkę zatwierdzeń](#approval-workflow).

### Ślad audytu przy każdej akcji

Każde działanie agenta jest rejestrowane z krótkim uzasadnieniem (1–2 zdania wyjaśniającego dlaczego) oraz wynikiem ufności (0.0–1.0). Oba pojawiają się w [Widoku szczegółów uruchomienia](#run-detail-view) oraz przy każdym [zatwierdzeniu](#approval-workflow). Przeszukiwanie pamięci jest jedynym wyjątkiem tylko do odczytu: nie jest zapisywane jako akcja i jest zawsze dostępne niezależnie od listy dozwolonych.

### Referencja narzędzi

#### Publikowanie komentarzy

Pozwala agentowi opublikować komentarz jako on sam. Komentarz jest wyświetlany publicznie pod nazwą wyświetlaną agenta. Używane przez agentów powitania i podsumowywania. Odwracalne — każdy moderator może usunąć nieodpowiedni komentarz. Zazwyczaj dozwolone bez zatwierdzenia; ogranicz je, jeśli twoja społeczność wymaga, aby każda wiadomość skierowana do publiczności była sprawdzana przez człowieka.

#### Głosowanie w komentarzach

Pozwala agentowi zagłosować za lub przeciw komentarzowi. Głos liczy się do sumy głosów komentarza jak każdy inny. Większość społeczności woli, aby boty nie brały udziału w głosowaniu; nie jest to włączone w żadnym szablonie startowym. Jeśli je włączysz, głosowanie jest odwracalne.

#### Przypięcie / odpięcie komentarza

Pozwala agentowi przypiąć komentarz na górze strony lub odpiąć już przypięty. Platforma nie wymusza zasady jednego przypięcia na wątek, więc agent przypinający powinien najpierw odpiąć poprzedni przypięty komentarz. Używane przez Top Comment Pinner template. Odwracalne; zazwyczaj dozwolone bez zatwierdzenia.

#### Zablokuj / odblokuj komentarz

Pozwala agentowi zapobiec dalszym odpowiedziom pod komentarzem lub przywrócić możliwość odpowiadania. Zablokowany komentarz pozostaje widoczny. Przydatne na czas ochłodzenia w zaognionych wątkach, w połączeniu z odroczonym odblokowaniem. Odwracalne, ale widoczne dla twojej społeczności; rozważ ograniczenie za zatwierdzeniem w społecznościach o dużych stawkach.

#### Oznacz / usuń oznaczenie spamu

Pozwala agentowi oznaczyć komentarz jako spam (ukrywając go przed czytelnikami i zasilając klasyfikator spamu) lub usunąć to oznaczenie. Podstawowe narzędzie dla każdego agenta moderującego. Odwracalne. Zdecydowanie rozważ ograniczenie zatwierdzeniem w pierwszych tygodniach, zanim zbudujesz zaufanie do agenta.

#### Zatwierdź / cofnij zatwierdzenie komentarza

Pozwala agentowi pokazać zatrzymany komentarz czytelnikom albo ukryć już widoczny. Najbardziej użyteczne w tenantach, które wstrzymują nowe komentarze do przeglądu moderatora. Duże konsekwencje przy cofnięciu zatwierdzenia widocznego komentarza — rozważ ograniczenie zatwierdzeniem.

#### Oznacz komentarz jako przejrzany

Narzędzie stanu kolejki: oznacza komentarz jako „moderator (lub agent) to obejrzał.” Nie zmienia widoczności. Niskie ryzyko; rzadko ograniczane.

#### Przyznaj odznakę

Pozwala agentowi przyznać użytkownikowi odznakę z konfiguracji odznak twojego tenant. Odwracalne przez moderatora. Rzadko ograniczane. Agent musi znać ID odznaki, więc uwzględnij odpowiednie ID w swoich [wytycznych społeczności](#community-guidelines) lub [początkowym poleceniu](#personality-prompt).

#### Wyślij e-mail

Pozwala agentowi wysłać tekstowy e-mail z adresu `noreply@fastcomments.com` na wybrany przez niego adres. Używaj oszczędnie — e-mail to narzędzie o najwyższej barierze i błędnych wiadomości trudno cofnąć. Zdecydowanie rozważ ograniczenie zatwierdzeniem i kieruj e-maile dotyczące zatwierdzeń do osoby, która zarządza skrzynką, do której agent będzie pisać.

#### Zapisz / przeszukaj pamięć agenta

Dwa powiązane narzędzia do odczytu i zapisu współdzielonej puli notatek o użytkowniku, dla którego wyzwalacz został uruchomiony. Pamięć jest współdzielona między wszystkimi agentami w twoim tenant, więc notatki agenta triage informują decyzje agenta-moderatora. Wyszukiwanie jest tylko do odczytu i zawsze dostępne; zapisywanie rzadko jest ograniczane. Zobacz [System pamięci agenta](#agent-memory-system) po pełny opis.

#### Ostrzeż użytkownika

Wysyła prywatną wiadomość DM z ostrzeżeniem do użytkownika dotyczące konkretnego komentarza i atomowo zapisuje to ostrzeżenie w pamięci agenta. Polityka eskalacji platformy opiera się na tym narzędziu — najpierw ostrzeżenie, ban tylko jeśli użytkownik ponownie naruszy zasady. Rzadziej ograniczane niż `ban_user`, ale rozważ ograniczenie w pierwszych tygodniach działania agenta. Zobacz [Ostrzeż użytkownika](#tool-warn-user) po pełną stronę.

#### Zbanuj użytkownika

Najpoważniejsze narzędzie, które agent może wywołać. Banuje użytkownika na określony czas, opcjonalnie jako shadow ban, opcjonalnie także blokując IP, opcjonalnie także usuwając wszystkie komentarze użytkownika. Dwie destrukcyjne opcje (IP, delete-all) są objęte dodatkowymi zgodami na formularzu edycji. W regionie UE wszystkie bany wymagają zatwierdzenia przez człowieka (zobacz [Zgodność z artykułem 17 DSA UE](#eu-dsa-compliance)). Zdecydowanie rozważ ograniczenie zatwierdzeniem wszędzie. Zobacz [Zbanuj użytkownika](#tool-ban-user) po pełną stronę.

### Ban-tool sub-options

Narzędzie Ban expose dwie destrukcyjne opcje - delete-all-comments i ban-by-IP - które są całkowicie ukryte przed modelem do momentu, gdy je włączysz przez sekcję **Ban options** w formularzu edycji. Nawet jeśli model zmyśli parametr, platforma odrzuca wartości, na które się nie zdecydowałeś. Zobacz [Zbanuj użytkownika](#tool-ban-user).