**Narzędzia** agenta to akcje, które może podejmować. Formularz edycji agenta zawiera sekcję **Allowed tool calls**, w której zaznaczasz narzędzia, z których agent może korzystać, oraz sekcję **Approvals**, w której zaznaczasz akcje wymagające zatwierdzenia przez człowieka, zanim wejdą w życie.

Każde narzędzie ma trzy poziomy:

- **Disallowed** - agent nie może go widzieć ani używać.
- **Allowed, no approval** - agent używa go bezpośrednio. Zapisywane w historii uruchomień.
- **Allowed, with approval** - wywołanie agenta trafia do kolejki do weryfikacji przez człowieka i wykonuje się dopiero po zatwierdzeniu.

Narzędzia niedozwolone są ciche: agent nie może o nie prosić, a platforma odmawia ich wykonania. Narzędzia z bramką zatwierdzania zawsze przechodzą przez [skrzynkę zatwierdzeń](#approval-workflow).

### Ślad audytu dla każdej akcji

Każda akcja agenta jest rejestrowana wraz z krótkim uzasadnieniem (1-2 zdania wyjaśniające dlaczego) oraz wskaźnikiem pewności (0.0-1.0). Oba elementy pojawiają się w [widoku szczegółów uruchomienia](#run-detail-view) i przy każdym [zatwierdzeniu](#approval-workflow). Wyszukiwanie w pamięci to jedyny wyjątek tylko-do-odczytu: nie jest rejestrowane jako akcja i jest zawsze dostępne niezależnie od listy dozwolonych.

### Referencja narzędzi

#### Publikowanie komentarzy

Pozwala agentowi opublikować komentarz we własnym imieniu. Komentarz jest wyświetlany publicznie pod nazwą wyświetlaną agenta. Używane przez agentów powitalnych i podsumowujących. Odwracalne - każdy moderator może usunąć zły komentarz. Włącz bramkę zatwierdzania, jeśli Twoja społeczność wymaga, aby każda publiczna wiadomość była weryfikowana przez człowieka.

#### Edycja komentarza

Pozwala agentowi przepisać tekst komentarza w jego zakresie. Oryginalny tekst jest zachowywany w dzienniku audytu komentarza. Zarezerwowane dla wąskich przypadków - redagowania wyciekniętych danych osobowych (PII) lub poprawiania wcześniejszej odpowiedzi samego agenta. Nie służy do przepisywania opinii czy łagodzenia tonu. Zobacz [Edit comment](#tool-edit-comment) dla pełnej strony.

#### Głosowanie na komentarze

Pozwala agentowi głosować za lub przeciw na komentarz. Głos liczy się do sumy głosów komentarza jak każdy inny. Większość społeczności woli, aby boty nie głosowały; nie jest włączone w żadnym szablonie startowym. Jeśli na to zezwolisz, głosowanie jest odwracalne.

#### Przypięcie / odpięcie komentarza

Pozwala agentowi przypiąć komentarz na górze strony lub odpiąć już przypięty. Platforma nie wymusza zasady jednego przypięcia na wątek, więc agent przypinający powinien otrzymać instrukcję, aby najpierw odpiąć poprzednio przypięty komentarz. Aby dowiedzieć się, co jest już przypięte na tej samej stronie, agent może wywołać narzędzie tylko-do-odczytu `get_pinned_comments` (patrz niżej). Używane przez szablon Top Comment Pinner.

#### Zablokowanie / odblokowanie komentarza

Pozwala agentowi zablokować dalsze odpowiedzi pod komentarzem lub je przywrócić. Zablokowany komentarz pozostaje widoczny. Przydatne do wyciszenia gorących wątków, w połączeniu z odroczonym odblokowaniem. Aby dowiedzieć się, co jest obecnie zablokowane na tej samej stronie, agent może wywołać narzędzie tylko-do-odczytu `get_locked_comments` (patrz niżej).

#### Oznaczenie / odznaczenie spamu

Pozwala agentowi oznaczyć komentarz jako spam (ukrywając go przed czytelnikami i zasilając klasyfikator spamu) lub usunąć tę flagę. Podstawowe narzędzie każdego agenta moderującego. Odwracalne.

#### Zatwierdzenie / cofnięcie zatwierdzenia komentarza

Pozwala agentowi pokazać czytelnikom komentarz trzymany w kolejce lub ukryć już widoczny. Najbardziej przydatne w tenantach, które trzymają nowe komentarze do weryfikacji moderatora.

#### Oznaczenie komentarza jako przejrzany

Narzędzie stanu kolejki: oznacza komentarz jako "moderator (lub agent) już to obejrzał". Nie zmienia widoczności. Niskie ryzyko; rzadko ograniczane zatwierdzeniem.

#### Nadanie odznaki

Pozwala agentowi nadać użytkownikowi odznakę skonfigurowaną dla Twojego tenanta. Odwracalne przez moderatora. Gdy to narzędzie jest włączone, agent widzi odznaki Twojego tenanta i sam wybiera właściwą, więc nie musisz wklejać identyfikatorów odznak do wytycznych społeczności ani początkowego promptu. Aby sterować tym, która odznaka jest nadawana za jakie zachowanie, odnoś się do odznak przez ich **Display Label** w prompcie.

#### Wysyłanie e-maila

Pozwala agentowi wysłać e-mail tekstowy do autora komentarza w zakresie triggera. Agent nigdy nie widzi adresu e-mail odbiorcy - wybiera komentarz, a platforma dostarcza wiadomość na adres, który komentujący podał przy publikacji. Adres nadawcy to markowy nadawca Twojego tenanta (z DKIM), gdy domena komentarza pasuje do skonfigurowanej domeny, w przeciwnym razie domyślny adres platformy. Używaj oszczędnie - e-mail to narzędzie o najwyższej "tarcia", a złe e-maile trudno cofnąć.

#### Zapis / wyszukiwanie pamięci agenta

Dwa sparowane narzędzia, które odczytują i zapisują wspólną pulę notatek o użytkowniku, dla którego trigger został uruchomiony. Pamięć jest współdzielona przez wszystkie agenty w Twoim tenancie, więc notatki agenta triage informują decyzje agenta moderującego. Wyszukiwanie jest tylko-do-odczytu i zawsze dostępne; zapis rzadko jest ograniczany zatwierdzeniem. Zobacz [Agent Memory System](#agent-memory-system) dla pełnego opisu.

#### Get pinned comments / Get locked comments

Dwa narzędzia odkrywania tylko-do-odczytu, które listują przypięte (lub zablokowane) komentarze na tej samej stronie (`urlId`), na której trigger został uruchomiony. Nie przyjmują argumentów - strona jest odczytywana z kontekstu triggera, więc agent nie może przeskoczyć na inne strony. Używaj ich, gdy agent ma działać na komentarzu, który jest już przypięty lub zablokowany - zazwyczaj jako pierwsze wywołanie przed `unpin_comment` lub `unlock_comment`, albo przed przypięciem nowego komentarza, aby najpierw odpiąć istniejący.

Każde narzędzie jest ograniczane osobno w **Allowed tool calls** (administrator zaznacza `List pinned comments on the current page` lub `List locked comments on the current page`). Nie można ich ograniczyć zatwierdzeniem - narzędzia tylko-do-odczytu nie mają skutków ubocznych do zatwierdzenia. Ich wywołanie nie jest rejestrowane jako akcja w historii uruchomień; pojawia się tylko wynikowe wywołanie `unpin_comment` / `unlock_comment` / `pin_comment` (jeśli wystąpi). Lista jest ograniczona do 20 najnowszych dopasowań na wywołanie.

Ważne do zrozumienia: gdy jedno z tych narzędzi zwraca commentId, ten commentId jest dodawany do zakresu agenta na dane uruchomienie, więc kolejne wywołanie `unpin_comment` / `unlock_comment` jest weryfikowane przez zabezpieczenie platformy sprawdzające cel narzędzia. Bez wcześniejszego wywołania narzędzia odkrywania agent nie może działać na komentarzach, które nie są już w bezpośrednim zakresie triggera. Dlatego agent typu "odpinający" zazwyczaj ma włączone oba narzędzia (np. `get_pinned_comments` plus `unpin_comment`).

#### Ostrzeżenie użytkownika

Wysyła prywatne ostrzeżenie DM do użytkownika dotyczące konkretnego komentarza i atomowo zapisuje ostrzeżenie w pamięci agenta. Polityka eskalacji platformy jest zbudowana wokół tego narzędzia - najpierw ostrzeżenie, ban tylko gdy użytkownik ponowi wykroczenie. Zobacz [Warn user](#tool-warn-user) dla pełnej strony.

#### Zbanowanie użytkownika

Najbardziej doniosłe narzędzie, jakie agent może wywołać. Banuje użytkownika na stały okres, opcjonalnie jako shadow ban, opcjonalnie także banując IP, opcjonalnie także usuwając wszystkie komentarze użytkownika. Dwie destrukcyjne opcje (IP, usuń-wszystko) są ukryte za dodatkowymi opt-inami w formularzu edycji. W regionie UE wszystkie bany wymagają zatwierdzenia przez człowieka (zobacz [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Zobacz [Ban user](#tool-ban-user) dla pełnej strony.

### Opcje dodatkowe narzędzia bana

Narzędzie bana udostępnia dwie destrukcyjne opcje - delete-all-comments i ban-by-IP - które są całkowicie ukryte przed modelem, dopóki ich nie włączysz w sekcji **Ban options** formularza edycji. Nawet jeśli model halucynuje parametr, platforma odrzuca wartości, na które nie wyraziłeś zgody. Zobacz [Ban user](#tool-ban-user).