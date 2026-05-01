Narzędzia agenta to akcje, które może wykonać. Formularz edycji agenta ma sekcję **Allowed tool calls**, gdzie zaznaczasz narzędzia, do których agent ma dostęp, oraz sekcję **Approvals**, gdzie zaznaczasz akcje, które powinny wymagać zatwierdzenia przez człowieka, zanim wejdą w życie.

Dla każdego narzędzia istnieją trzy poziomy:

- **Disallowed** - agent nie może go zobaczyć ani używać.
- **Allowed, no approval** - agent używa go bezpośrednio. Zapisane w historii uruchomień.
- **Allowed, with approval** - wywołanie agenta trafia do kolejki do przeglądu przez człowieka i wykona się dopiero po zatwierdzeniu.

Narzędzia oznaczone jako disallowed są ciche: agent nie może o nie prosić, a platforma odrzuca je wprost. Narzędzia wymagające zatwierdzenia zawsze przechodzą przez [approvals inbox](#approval-workflow).

### Ślad audytu dla każdej akcji

Każda akcja wykonywana przez agenta jest rejestrowana z krótkim uzasadnieniem (1–2 zdania wyjaśniające dlaczego) oraz współczynnikiem pewności (0.0–1.0). Obie informacje pojawiają się w [Run Detail View](#run-detail-view) oraz przy każdym [approval](#approval-workflow). Przeszukiwanie pamięci jest jedynym wyjątkiem tylko do odczytu: nie jest rejestrowane jako akcja i jest zawsze dostępne niezależnie od listy dozwolonych narzędzi.

### Referencje narzędzi

#### Posting comments

Pozwala agentowi opublikować komentarz we własnym imieniu. Komentarz jest wyświetlany publicznie pod nazwą wyświetlaną agenta. Używane przez agentów witających i podsumowujących. Odwracalne - każdy moderator może usunąć niewłaściwy komentarz. Zablokuj za zatwierdzeniem, jeśli twoja społeczność wymaga, aby każda publiczna wiadomość była sprawdzana przez człowieka.

#### Editing a comment

Pozwala agentowi przepisać treść komentarza objętego zakresem. Oryginalny tekst jest zachowany w rejestrze audytu komentarza. Zarezerwuj do wąskich przypadków - redagowania ujawnionych przez użytkownika danych osobowych lub poprawiania własnej wcześniejszej odpowiedzi agenta. Nie do przepisywania opinii ani łagodzenia tonu. Zobacz [Edit comment](#tool-edit-comment) dla pełnej strony.

#### Voting on comments

Pozwala agentowi głosować za lub przeciw komentarzowi. Głos liczy się do ogólnej liczby głosów komentarza jak każdy inny głos. Większość społeczności woli, aby boty nie głosowały; nie jest włączone w żadnym szablonie startowym. Jeśli to umożliwisz, głosowanie jest odwracalne.

#### Pin / unpin a comment

Pozwala agentowi przypiąć komentarz na górze strony lub odpiąć już przypięty. Platforma nie wymusza zasady jednego przypięcia na wątek, więc agent przypinający powinien zostać poinstruowany, aby najpierw odpiąć poprzedni przypięty komentarz. Aby odkryć, co już jest przypięte na tej samej stronie, agent może wywołać narzędzie tylko do odczytu `get_pinned_comments` (patrz poniżej). Używane przez szablon Top Comment Pinner.

#### Lock / unlock a comment

Pozwala agentowi uniemożliwić dalsze odpowiedzi pod komentarzem lub przywrócić możliwość odpowiadania. Zablokowany komentarz pozostaje widoczny. Przydatne do ostudzenia rozgrzanych wątków, w parze z opóźnionym odblokowaniem. Aby odkryć, co jest obecnie zablokowane na tej samej stronie, agent może wywołać narzędzie tylko do odczytu `get_locked_comments` (patrz poniżej).

#### Mark / unmark spam

Pozwala agentowi oznaczyć komentarz jako spam (ukrywając go przed czytelnikami i dostarczając dane do klasyfikatora spamu) lub usunąć taką flagę. Podstawowe narzędzie dla każdego agenta moderacji. Odwracalne.

#### Approve / un-approve a comment

Pozwala agentowi pokazać zatrzymany komentarz czytelnikom albo ukryć już widoczny. Najbardziej przydatne w tenantach, które zatrzymują nowe komentarze do przeglądu moderatora.

#### Mark a comment reviewed

Narzędzie stanu kolejki: oznacza komentarz jako „moderator (lub agent) obejrzał to”. Nie zmienia widoczności. Niskie ryzyko; rzadko wymaga zatwierdzenia.

#### Award a badge

Pozwala agentowi przyznać użytkownikowi odznakę, którą skonfigurowałeś dla swojego tenanta. Odwracalne przez moderatora. Gdy to narzędzie jest włączone, agent może zobaczyć odznaki twojego tenanta i samodzielnie wybrać odpowiednią, więc nie musisz wklejać identyfikatorów odznak w wytyczne społeczności ani w początkowy prompt. Aby nakierować, która odznaka ma być przyznana za jakie zachowanie, odwołuj się do odznak przez ich **Display Label** w promptcie.

#### Send email

Pozwala agentowi wysłać wiadomość tekstową e-mail do autora komentarza w zakresie wyzwalacza. Agent nigdy nie widzi adresu e-mail odbiorcy - wybiera komentarz, a platforma doręcza na adres, który komentujący podał przy publikacji. Adres nadawcy to spersonalizowany nadawca twojego tenanta (z DKIM), gdy domena komentarza pasuje do skonfigurowanej domeny, w przeciwnym razie używany jest domyślny nadawca platformy. Używaj oszczędnie - e-mail to narzędzie o największym tarciu, a złe e-maile trudno cofnąć.

#### Save / search agent memory

Dwa sprzężone narzędzia, które odczytują i zapisują wspólny zbiór notatek o użytkowniku, dla którego wywołał się wyzwalacz. Pamięć jest współdzielona pomiędzy wszystkimi agentami w twoim tenancie, więc notatki agenta triage wpływają na decyzje agenta moderującego. Wyszukiwanie jest tylko do odczytu i zawsze dostępne; zapisywanie rzadko wymaga zatwierdzenia. Zobacz [Agent Memory System](#agent-memory-system) dla pełnego projektu.

#### Get pinned comments / Get locked comments

Dwa narzędzia tylko do odczytu, które wypisują przypięte (lub zablokowane) komentarze na tej samej stronie (`urlId`), na której uruchomił się wyzwalacz. Nie przyjmują argumentów - strona jest odczytywana z kontekstu wyzwalacza, więc agent nie może przełączyć się na inne strony. Użyj ich, gdy agent musi działać na komentarzu, który jest już przypięty lub zablokowany - zazwyczaj jest to pierwsze wywołanie przed `unpin_comment` lub `unlock_comment`, albo przed przypięciem nowego komentarza, aby najpierw odpiąć istniejący.

Każde z tych narzędzi jest oddzielnie kontrolowane w **Allowed tool calls** (administrator zaznacza `List pinned comments on the current page` lub `List locked comments on the current page`). Nie mogą być objęte zatwierdzeniem - narzędzia tylko do odczytu nie mają skutków ubocznych wymagających zatwierdzenia. Wywołanie ich nie jest rejestrowane jako akcja w historii uruchomień; tylko wynikowe wywołanie `unpin_comment` / `unlock_comment` / `pin_comment` (jeśli wystąpi) pojawia się w historii. Lista jest ograniczona do 20 najnowszych dopasowań na wywołanie.

Ważne do zrozumienia: gdy jedno z tych narzędzi zwraca commentId, ten commentId zostaje dodany do zakresu per-run agenta, więc następcze wywołanie `unpin_comment` / `unlock_comment` jest weryfikowane względem mechanizmu bezpieczeństwa targetów narzędzi platformy. Bez wcześniejszego wywołania narzędzia odkrywającego agent nie może działać na komentarzach, które nie znajdują się już w bezpośrednim zakresie wyzwalacza. Dlatego agent typu unpin zwykle ma włączone oba narzędzia (np. `get_pinned_comments` oraz `unpin_comment`).

#### Warn a user

Wysyła prywatną wiadomość DM ostrzegającą użytkownika o konkretnym komentarzu i atomowo zapisuje ostrzeżenie w pamięci agenta. Polityka eskalacji platformy opiera się na tym narzędziu - najpierw ostrzeżenie, ban tylko w przypadku ponownego naruszenia. Zobacz [Warn user](#tool-warn-user) dla pełnej strony.

#### Ban a user

Najpoważniejsze narzędzie, które agent może wywołać. Banuje użytkownika na określony czas, opcjonalnie jako shadow ban, opcjonalnie również blokując IP, opcjonalnie również usuwając wszystkie komentarze użytkownika. Dwie destrukcyjne opcje (IP, usuń-wszystko) są zabezpieczone dodatkowymi zgodami w formularzu edycji. W regionie UE wszystkie bany wymagają zatwierdzenia przez człowieka (patrz [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Zobacz [Ban user](#tool-ban-user) dla pełnej strony.

### Opcje dodatkowe narzędzia Ban

Narzędzie Ban udostępnia dwie destrukcyjne opcje - delete-all-comments i ban-by-IP - które są całkowicie ukryte przed modelem, dopóki ich nie włączysz przez sekcję **Ban options** w formularzu edycji. Nawet jeśli model zhalucynuje parametr, platforma odrzuci wartości, których nie włączyłeś. Zobacz [Ban user](#tool-ban-user).