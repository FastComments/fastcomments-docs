Narzędzia agenta (**tools**) to akcje, które może wykonać. Formularz edycji agenta zawiera sekcję **Allowed tool calls**, w której zaznaczasz narzędzia, których agent może używać, oraz sekcję **Approvals**, w której zaznaczasz akcje, które powinny wymagać zatwierdzenia przez człowieka, zanim wejdą w życie.

Istnieją trzy poziomy dla dowolnego narzędzia:

- **Disallowed** - agent nie może go zobaczyć ani używać.
- **Allowed, no approval** - agent używa go bezpośrednio. Zapisuje się w historii uruchomień.
- **Allowed, with approval** - wywołanie agenta trafia do kolejki do przeglądu przez człowieka i uruchamia się dopiero po jego zatwierdzeniu.

Narzędzia oznaczone jako Disallowed są ciche: agent nie może o nie prosić, a platforma odrzuca takie żądania wprost. Narzędzia wymagające zatwierdzenia zawsze przechodzą przez [approvals inbox](#approval-workflow).

### Ślad audytu przy każdej akcji

Każda akcja wykonana przez agenta jest rejestrowana z krótkim uzasadnieniem (1–2 zdania wyjaśniające dlaczego) oraz oceną pewności (0.0–1.0). Oba te elementy pojawiają się w [Run Detail View](#run-detail-view) i przy każdym [approval](#approval-workflow). Wyszukiwanie w pamięci jest jedynym wyjątkiem tylko do odczytu: nie jest rejestrowane jako akcja i jest zawsze dostępne bez względu na allowlist.

### Referencja narzędzi

#### Posting comments

Pozwala agentowi opublikować komentarz jako on sam. Komentarz jest publicznie widoczny pod nazwą wyświetlaną agenta. Używane przez agentów typu greeter i summarizer. Cofalne — każdy moderator może usunąć zły komentarz. Zazwyczaj dozwolone bez zatwierdzenia; jeśli twoja społeczność wymaga, aby każda publiczna wiadomość była sprawdzana przez człowieka, zabezpiecz to zatwierdzeniem.

#### Editing a comment

Pozwala agentowi przepisać tekst komentarza objętego zakresem. Oryginalny tekst jest zachowany w dzienniku audytu komentarza. Zarezerwuj na wąskie przypadki — zaciemnianie PII ujawnionego przez użytkownika lub poprawianie poprzedniej odpowiedzi agenta. Nie do przepisywania opinii czy łagodzenia tonu. **Zdecydowanie rozważ zabezpieczenie tego za pomocą zatwierdzenia.** Zobacz stronę [Edit comment](#tool-edit-comment) po pełne informacje.

#### Voting on comments

Pozwala agentowi głosować za lub przeciw komentarzowi. Głos liczy się do sumy głosów komentarza jak każdy inny głos. Większość społeczności woli, aby boty nie głosowały; nie jest to włączone w żadnym starterowym szablonie. Jeśli jednak na to pozwolisz, głosowanie jest cofane.

#### Pin / unpin a comment

Pozwala agentowi przypiąć komentarz na górze strony lub odpiąć już przypięty. Platforma nie wymusza zasady jednego przypięcia na wątek, więc agenta przypinającego należy poinstruować, aby najpierw odpiął poprzednio przypięty komentarz. Używane przez szablon Top Comment Pinner. Cofalne; zwykle dozwolone bez zatwierdzenia.

#### Lock / unlock a comment

Pozwala agentowi zablokować możliwość dalszych odpowiedzi pod komentarzem lub przywrócić odpowiedzi. Zablokowany komentarz pozostaje widoczny. Przydatne do schładzania gorących wątków, w połączeniu z odroczonym odblokowaniem. Cofalne, ale widoczne dla twojej społeczności; rozważ zabezpieczenie zatwierdzeniem w społecznościach o wysokiej stawce.

#### Mark / unmark spam

Pozwala agentowi oznaczyć komentarz jako spam (ukrywając go przed czytelnikami i przekazując do klasyfikatora spamu) lub usunąć ten znacznik. Podstawowe narzędzie dla każdego agenta moderacyjnego. Cofalne. Zdecydowanie rozważ zabezpieczenie zatwierdzeniem w pierwszych tygodniach, dopóki nie zbudujesz zaufania do agenta.

#### Approve / un-approve a comment

Pozwala agentowi pokazać wstrzymany komentarz czytelnikom lub ukryć już widoczny. Najbardziej użyteczne w tenantach, które wstrzymują nowe komentarze do przeglądu moderatora. Wysokie ryzyko przy cofnięciu zatwierdzenia widocznego komentarza — rozważ zabezpieczenie zatwierdzeniem.

#### Mark a comment reviewed

Narzędzie stanu kolejki: oznacza komentarz jako „moderator (lub agent) przyjrzał się temu”. Nie zmienia widoczności. Niskie ryzyko; rzadko zabezpieczane.

#### Award a badge

Pozwala agentowi przyznać użytkownikowi odznakę z konfiguracji odznak tenant'a. Cofalne przez moderatora. Rzadko zabezpieczane. Agent musi znać ID odznaki, więc dołącz odpowiednie ID w swoich [community guidelines](#community-guidelines) lub [initial prompt](#personality-prompt).

#### Send email

Pozwala agentowi wysłać wiadomość e-mail w zwykłym tekście z `noreply@fastcomments.com` na adres, który wybierze. Używaj oszczędnie — e-mail to narzędzie o najwyższej barierze i złe e-maile trudno cofnąć. Zdecydowanie rozważ zabezpieczenie zatwierdzeniem i kieruj e-maile zatwierdzające do osób, które zarządzają skrzynką, na którą agent będzie wysyłać wiadomości.

#### Save / search agent memory

Dwa powiązane narzędzia, które odczytują i zapisują wspólną pulę notatek o użytkowniku, dla którego wywołano trigger. Pamięć jest współdzielona między wszystkimi agentami w twoim tenant, więc notatki agenta triage wpływają na decyzje agenta moderatora. Search jest tylko do odczytu i zawsze dostępne; saving rzadko jest zabezpieczane. Zobacz [Agent Memory System](#agent-memory-system) po pełny opis.

#### Warn a user

Wysyła prywatną wiadomość DM ostrzegającą użytkownika o konkretnym komentarzu i atomowo zapisuje ostrzeżenie w pamięci agenta. Polityka eskalacji platformy opiera się na tym narzędziu — najpierw ostrzeżenie, ban tylko jeśli użytkownik naruszy ponownie. Mniej często zabezpieczane niż `ban_user`, ale rozważ zabezpieczenie w pierwszych tygodniach życia agenta. Zobacz stronę [Warn user](#tool-warn-user) po pełne informacje.

#### Ban a user

Najpoważniejsze narzędzie, jakie agent może wywołać. Banuje użytkownika na określony czas, opcjonalnie jako shadow ban, opcjonalnie także blokując IP, opcjonalnie także usuwając wszystkie komentarze użytkownika. Dwie destrukcyjne opcje (IP, delete-all) są zabezpieczone dodatkowymi zgodami na formularzu edycji. W regionie UE wszystkie bany wymagają zatwierdzenia przez człowieka (zobacz [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Zdecydowanie rozważ zabezpieczenie zatwierdzeniem wszędzie. Zobacz stronę [Ban user](#tool-ban-user) po pełne informacje.

### Podopcje narzędzia Ban

Narzędzie Ban udostępnia dwie destrukcyjne opcje — delete-all-comments i ban-by-IP — które są ukryte przed modelem całkowicie, dopóki nie włączysz ich przez sekcję **Ban options** w formularzu edycji. Nawet jeśli model zhalucynuje parametr, platforma odrzuci wartości, których nie włączyłeś. Zobacz [Ban user](#tool-ban-user).