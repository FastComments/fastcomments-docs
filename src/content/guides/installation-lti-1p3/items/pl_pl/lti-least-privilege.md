Integracja FastComments LTI 1.3 stosuje zasadę najmniejszych uprawnień: używa tylko tych roszczeń z uruchomienia, które są wymagane do zidentyfikowania użytkownika, przypisania komentarzy do właściwego kursu i zasobu oraz zastosowania uprawnień opartych na rolach.

Reszta tej strony mapuje każde roszczenie używane przez integrację, każdą usługę LTI Advantage, o którą nie występuje żądanie, oraz każdą kategorię danych, których nie zbiera. Recenzenci ds. bezpieczeństwa i zamówień mogą bezpośrednio wykorzystać odpowiedzi z tabel poniżej.

## Elementy danych otrzymywane od LMS

Każde uruchomienie LTI 1.3 zawiera podpisany JWT od LMS. FastComments wyodrębnia z tego JWT następujące roszczenia i nie używa niczego więcej:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Identyfikuje użytkownika konsekwentnie między uruchomieniami, tak aby ta sama osoba była rozpoznawana jako ten sam użytkownik SSO FastComments | Tak | Tak, jako część stabilnego wewnętrznego ID SSO |
| Display name | `name` | Atrybucja wyświetlana obok komentarzy użytkownika | Tak (zwraca „Użytkownik LMS”, jeżeli brak) | Tak |
| Email | `email` | Dopasowanie konta, powiadomienia, moderacja, korespondencja wsparcia | Opcjonalny (integracja działa bez niego) | Tak, jeśli dostarczony |
| Avatar URL | `picture` | Wyświetlane przy komentarzach użytkownika | Opcjonalny | Tylko URL; FastComments nie pobiera ani nie hostuje ponownie obrazu |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Określa, czy użytkownik jest administratorem, instruktorem (moderator), czy uczestnikiem | Tak | Wyprowadzone flagi `isAdmin` / `isModerator` na sesji SSO |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Wiąże wątek komentarzy z właściwym kursem w LMS | Tak | Tak, jako część rozwiązanej identyfikacji strony |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Przypisuje komentarze do właściwej aktywności lub miejsca narzędzia w kursie | Tak, jeśli obecny | Tak, jako część rozwiązanej identyfikacji strony |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Kieruje uruchomienie do właściwej konfiguracji dzierżawy FastComments | Tak | Tak, w rekordzie konfiguracji LTI FastComments |

## Roszczenia i zakresy zadeklarowane podczas rejestracji

Podczas dynamicznej rejestracji LTI 1.3 FastComments rejestruje się z `scope: ""` (bez dodatkowych zakresów OAuth) i deklaruje tylko te roszczenia OpenID Connect:

`iss`, `sub`, `name`, `email`, `picture`

Rejestruje dwa typy wiadomości:

- `LtiResourceLinkRequest` - standardowe uruchomienie kursu do FastComments.
- `LtiDeepLinkingRequest` - pozwala instruktorom umieścić narzędzie FastComments wewnątrz kursu.

Z LMS nie są żądane żadne dodatkowe tokeny dostępu.

## Usługi LTI Advantage, o które nie występuje żądanie

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Nie | Integracja nie potrzebuje listy uczestników kursu; tożsamość użytkownika jest dostarczana przy każdym uruchomieniu |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Nie | Integracja nie jest powiązana z dziennikiem ocen |
| Deep Linking beyond the standard placement return | Brak dodatkowych danych | Deep linking jest używany tylko do umieszczania narzędzia przez instruktora; żadne treści kursu nie są wyliczane |

## Dane, które nie są zbierane

Poza samym LTI, FastComments nie żąda ani nie otrzymuje następujących od LMS lub użytkownika:

| Category | Collected? |
|----------|------------|
| Student grades | Nie |
| Assignment submissions | Nie |
| Attendance records | Nie |
| Full course rosters | Nie |
| Government identifiers | Nie |
| Date of birth | Nie |
| Postal address or phone number | Nie |
| Financial information | Nie |
| LMS administrator credentials | Nie |

## Granice dostępu

- FastComments otrzymuje dane wyłącznie w ramach autoryzowanego uruchomienia LTI 1.3 podpisanego kluczami zarejestrowanymi przez LMS. Integracja nie dzwoni z powrotem do LMS po dodatkowe informacje.
- Tokeny uruchomienia są jednorazowe i krótkotrwałe. Zduplikowane lub wygasłe tokeny są odrzucane.
- Administratorzy LMS kontrolują, gdzie narzędzie jest wdrożone w ich platformie. D2L Brightspace, na przykład, obsługuje zakresowanie org-unit dla poszczególnych wdrożeń i ustawienia bezpieczeństwa na poziomie wdrożenia, co pozwala administratorom ograniczyć narzędzie do konkretnych kursów lub jednostek organizacyjnych zamiast udostępniać je globalnie. Moodle, Blackboard, Sakai i Schoology oferują równoważne kontrole na poziomie wdrożeń w swoich implementacjach LTI 1.3.

## Przechowywanie i retencja

FastComments przechowuje dane pochodzące z LTI przez okres aktywnej usługi komentowania i zgodnie z ustawieniami retencji skonfigurowanymi przez klienta. Dane komentarzy są przechowywane w produkcyjnym magazynie szyfrowanym w stanie spoczynku. Po zakończeniu konta lub na pisemne żądanie usunięcia FastComments usuwa lub anonimizuje dane klienta zgodnie z obowiązującą umową.

Szczegółowe informacje o przechowywaniu i przetwarzaniu danych znajdują się w <a href="https://fastcomments.com/privacy-policy" target="_blank">Polityce prywatności FastComments</a>.

## Częstotliwość przeglądu

Każda nowa funkcja LTI, która wymagałaby dodatkowych roszczeń, zakresów lub usług LTI Advantage, jest przeglądana przed wydaniem, aby potwierdzić, że żądany dostęp jest konieczny i proporcjonalny do dostarczanej funkcjonalności.

## Krótkie oświadczenie do ankiet bezpieczeństwa

> FastComments stosuje zasadę najmniejszych uprawnień i minimalizację danych w swojej integracji LTI 1.3. Integracja używa tylko roszczeń uruchomienia LTI wymaganych do uwierzytelnienia użytkownika (`sub`, `name`, `email`, `picture`), określenia jego roli oraz identyfikacji kursu i zasobu, do których należą komentarze. FastComments nie żąda Names and Role Provisioning Services, Assignment and Grade Services, danych dziennika ocen, obecności, pełnych list uczestników ani dostępu administracyjnego do LMS. Administratorzy LMS zachowują kontrolę nad tym, w których jednostkach organizacyjnych, kursach i wdrożeniach narzędzie jest dostępne.