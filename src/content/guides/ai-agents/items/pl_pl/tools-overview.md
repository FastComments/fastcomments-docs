Narzędzia agenta (**tools**) to akcje, które może on podjąć. Formularz edycji agenta ma sekcję **Dozwolone wywołania narzędzi** (Allowed tool calls), w której zaznaczasz narzędzia, z których agent może korzystać, oraz sekcję **Zatwierdzenia** (Approvals), w której zaznaczasz działania wymagające zatwierdzenia przez człowieka przed ich wejściem w życie.

Są trzy poziomy dla każdego narzędzia:

- **Zabronione** - agent nie może go zobaczyć ani użyć.
- **Dozwolone, bez zatwierdzenia** - agent używa go bezpośrednio. Rejestrowane w historii uruchomień.
- **Dozwolone, z zatwierdzeniem** - wywołanie agenta jest umieszczane w kolejce do przeglądu przez człowieka i uruchamiane tylko po jego zatwierdzeniu.

Zabronione narzędzia są ciche: agent nie może o nie prosić, a platforma odrzuca je wprost. Narzędzia wymagające zatwierdzenia zawsze przechodzą przez [skrzynkę zatwierdzeń](#approval-workflow).

### Ścieżka audytu dla każdej akcji

Każde działanie agenta jest rejestrowane z krótkim uzasadnieniem (1–2 zdania wyjaśniające dlaczego) oraz oceną pewności (0.0–1.0). Oba elementy pojawiają się w [Widoku szczegółów uruchomienia](#run-detail-view) i przy każdym [zatwierdzeniu](#approval-workflow). Przeszukiwanie pamięci jest jedynym wyjątkiem tylko do odczytu: nie jest rejestrowane jako działanie i jest zawsze dostępne niezależnie od listy dozwolonych narzędzi.

### Opis narzędzi

#### Dodawanie komentarzy

Pozwala agentowi dodać komentarz jako on sam. Komentarz jest pokazywany publicznie pod nazwą wyświetlaną agenta. Używane przez agentów powitalnych i podsumowujących. Odwracalne - każdy moderator może usunąć nieodpowiedni komentarz. Zazwyczaj dozwolone bez zatwierdzenia; wymuś zatwierdzenie, jeśli Twoja społeczność potrzebuje, aby każda publiczna wiadomość była sprawdzana przez człowieka.

#### Edytowanie komentarza

Pozwala agentowi przepisać tekst komentarza objętego zakresem. Oryginalny tekst jest zachowany w logu audytu komentarza. Stosuj tylko w wąskich przypadkach — redagowanie PII ujawnionego przez użytkownika lub poprawianie wcześniejszej odpowiedzi agenta. Nie do przepisywania opinii ani łagodzenia tonu. **Zdecydowanie rozważ objęcie tego wymogiem zatwierdzenia.** Zobacz [Edytuj komentarz](#tool-edit-comment) po pełne informacje.

#### Głosowanie na komentarzach

Pozwala agentowi głosować za lub przeciw komentarzowi. Głos liczy się do łącznej liczby głosów komentarza jak każdy inny głos. Większość społeczności woli, aby boty nie głosowały; nie jest to włączone w żadnym szablonie startowym. Jeśli to pozwolisz, głosowanie jest odwracalne.

#### Przypinanie / odpinanie komentarza

Pozwala agentowi przypiąć komentarz na górze strony lub odpiąć już przypięty. Platforma nie egzekwuje zasady jednego przypięcia na wątek, więc agent przypinający powinien najpierw odpiąć poprzedni przypięty komentarz. Używane przez Top Comment Pinner template. Odwracalne; zwykle dozwolone bez zatwierdzenia.

#### Blokowanie / odblokowywanie komentarza

Pozwala agentowi uniemożliwić dalsze odpowiedzi pod komentarzem lub przywrócić możliwość odpowiadania. Zablokowany komentarz pozostaje widoczny. Przydatne do ostudzenia rozpalonych wątków, w połączeniu z odroczonym odblokowaniem. Odwracalne, ale widoczne dla Twojej społeczności; rozważ objęcie zatwierdzeniem na społecznościach o dużym ryzyku.

#### Oznacz / usuń oznaczenie spamu

Pozwala agentowi oznaczyć komentarz jako spam (ukrywając go przed czytelnikami i dostarczając go do klasyfikatora spamu) lub usunąć to oznaczenie. Podstawowe narzędzie dla każdego agenta moderującego. Odwracalne. Zdecydowanie rozważ objęcie zatwierdzeniem w pierwszych tygodniach, gdy budujesz zaufanie do agenta.

#### Zatwierdź / cofnij zatwierdzenie komentarza

Pozwala agentowi pokazać wstrzymany komentarz czytelnikom lub ukryć już widoczny. Najbardziej przydatne w tenantach, które wstrzymują nowe komentarze do przeglądu moderatora. Duże ryzyko przy cofaniu zatwierdzenia widocznego komentarza — rozważ objęcie tego zatwierdzeniem.

#### Oznacz komentarz jako sprawdzony

Narzędzie stanu kolejki: oznacza komentarz jako „moderator (lub agent) przyjrzał się temu”. Nie zmienia widoczności. Niskie ryzyko; rzadko objęte zatwierdzeniem.

#### Przyznaj odznakę

Pozwala agentowi przyznać użytkownikowi odznakę skonfigurowaną dla Twojego tenanta. Odwracalne przez moderatora. Rzadko objęte zatwierdzeniem. Gdy to narzędzie jest włączone, agent widzi odznaki Twojego tenanta i sam wybiera odpowiednią, więc nie musisz wklejać identyfikatorów odznak do wytycznych społeczności ani do początkowego promptu. Jeśli chcesz wskazać, która odznaka jest przyznawana za jakie zachowanie, odwołuj się do odznak przez ich **Etykietę wyświetlaną** w promptcie.

#### Wysyłanie e-maili

Pozwala agentowi wysłać e-mail w formacie plain-text do autora komentarza w zakresie triggera. Agent nigdy nie widzi adresu e-mail odbiorcy — wybiera komentarz, a platforma dostarcza wiadomość na adres podany przez komentującego podczas publikacji. Adres nadawcy to brandowany nadawca Twojego tenanta (z DKIM), gdy domena komentarza pasuje do skonfigurowanej domeny, w przeciwnym razie używany jest domyślny nadawca platformy. Używaj oszczędnie — e-mail to narzędzie o najwyższej tarciwości, a złe e-maile trudno cofnąć. Zdecydowanie rozważ objęcie zatwierdzeniem i kieruj e-maile zatwierdzające do osoby odpowiedzialnej za skrzynkę, na którą agent ostatecznie będzie wysyłał wiadomości.

#### Zapisz / przeszukaj pamięć agenta

Dwa powiązane narzędzia, które odczytują i zapisują wspólną pulę notatek o użytkowniku, dla którego uruchomił się trigger. Pamięć jest współdzielona między wszystkimi agentami w Twoim tenancie, więc notatki agenta triage wpływają na decyzje agenta moderatora. Wyszukiwanie jest tylko do odczytu i zawsze dostępne; zapisywanie rzadko jest objęte zatwierdzeniem. Zobacz [System pamięci agenta](#agent-memory-system) po pełny opis.

#### Ostrzeż użytkownika

Wysyła prywatne DM ostrzegające użytkownika o konkretnym komentarzu i atomowo zapisuje ostrzeżenie w pamięci agenta. Polityka eskalacji platformy opiera się na tym narzędziu — najpierw ostrzegaj, blokuj tylko w razie ponownego przewinienia. Mniej powszechnie objęte zatwierdzeniem niż `ban_user`, ale rozważ objęcie zatwierdzeniem w pierwszych tygodniach życia agenta. Zobacz [Ostrzeż użytkownika](#tool-warn-user) po pełne informacje.

#### Zablokuj użytkownika

Najbardziej doniosłe narzędzie, które agent może wywołać. Zbanuje użytkownika na określony czas, opcjonalnie jako shadow ban, opcjonalnie również blokując IP, opcjonalnie również usuwając wszystkie komentarze użytkownika. Dwie destrukcyjne opcje (IP, usunięcie wszystkich) są dostępne tylko po dodatkowym włączeniu ich na formularzu edycji. W regionie UE wszystkie bany wymagają zatwierdzenia przez człowieka (zobacz [Zgodność z artykułem 17 DSA UE](#eu-dsa-compliance)). Zdecydowanie rozważ objęcie wymogiem zatwierdzenia wszędzie. Zobacz [Zablokuj użytkownika](#tool-ban-user) po pełne informacje.

### Podopcje narzędzia blokowania

Narzędzie blokowania udostępnia dwie destrukcyjne opcje — delete-all-comments i ban-by-IP — które są całkowicie ukryte przed modelem, dopóki nie włączysz ich w sekcji **Opcje blokowania** na formularzu edycji. Nawet jeśli model zmyśli parametr, platforma odrzuci wartości, których nie włączyłeś. Zobacz [Zablokuj użytkownika](#tool-ban-user).