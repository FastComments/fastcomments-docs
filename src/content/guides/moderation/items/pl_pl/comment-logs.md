FastComments automatycznie śledzi szczegółowe zdarzenia dla każdego komentarza, aby zapewnić przejrzystość decyzji moderacyjnych i działań systemu. Te logi pomagają zrozumieć, dlaczego komentarz został zatwierdzony, oznaczony jako spam lub zmieniono jego status.

## Dostęp do logów komentarzy

Aby wyświetlić logi dla konkretnego komentarza:

1. Przejdź do strony **Moderate Comments** w panelu FastComments
2. Znajdź komentarz, który chcesz przejrzeć
3. Kliknij przycisk **View Logs** (ikona zegara) w pasku akcji komentarza
4. Pojawi się okno dialogowe pokazujące pełną historię zdarzeń dla tego komentarza

Każdy wpis w logu wyświetla:
- **When** – Znacznik czasu zdarzenia
- **Who** – Użytkownik lub system, który wywołał zdarzenie (jeśli dotyczy)
- **What** – Typ akcji lub zdarzenia
- **Details** – Dodatkowy kontekst, taki jak wartości przed/po, nazwy silników lub powiązane dane

## Zdarzenia w logu komentarzy

Każdy komentarz utrzymuje log zdarzeń, które występują w jego cyklu życia. Poniżej znajdują się typy zdarzeń, które są śledzone:

### Zdarzenia anonimizacji
- **Anonymized** – Zawartość komentarza została wyczyszczona, a użytkownik oznaczony jako usunięty
- **RestoredFromAnonymized** – Komentarz został przywrócony ze stanu anonimowego

### Zdarzenia zatwierdzania
- **ApprovedDueToPastComment** – Komentarz zatwierdzony, ponieważ użytkownik wcześniej zatwierdzał komentarze (zawiera odniesienie do poprzedniego komentarza)
- **ApprovedIsAdmin** – Komentarz zatwierdzony, ponieważ użytkownik jest administratorem
- **NotApprovedRequiresApproval** – Komentarz wymaga ręcznego zatwierdzenia
- **NotApprovedLowTrustFactor** – Komentarz niezatwierdzony z powodu niskiego czynnika zaufania użytkownika (zawiera wartość czynnika zaufania)

### Zdarzenia zatwierdzania komentarzy profilowych
Te zdarzenia dotyczą konkretnie komentarzy na profilach użytkowników:
- **ApprovedProfileAutoApproveAll** – Komentarz profilowy automatycznie zatwierdzony, ponieważ właściciel profilu włączył automatyczne zatwierdzanie wszystkich komentarzy
- **ApprovedProfileTrusted** – Komentarz profilowy zatwierdzony, ponieważ komentujący jest zaufany (zawiera odniesienie do komentarza, który ustanowił zaufanie)
- **NotApprovedProfileManualApproveAll** – Komentarz profilowy wymaga ręcznego zatwierdzenia, ponieważ właściciel profilu włączył ręczne zatwierdzanie
- **NotApprovedProfileNotTrusted** – Komentarz profilowy niezatwierdzony, ponieważ komentujący nie jest zaufany
- **NotApprovedProfileNewUser** – Komentarz profilowy niezatwierdzony, ponieważ komentujący jest nowym użytkownikiem

### Zdarzenia wykrywania spamu
- **IsSpam** – Komentarz oznaczony jako spam przez silnik wykrywania (zawiera, który silnik podjął decyzję)
- **IsSpamDueToBadWords** – Komentarz oznaczony jako spam z powodu filtru wulgaryzmów
- **IsSpamFromLLM** – Komentarz oznaczony jako spam przez silnik AI/LLM (zawiera nazwę silnika, odpowiedź i liczbę tokenów)
- **IsSpamRepeatComment** – Komentarz oznaczony jako spam za powtarzalność (zawiera, który silnik to wykrył)
- **NotSpamIsOnlyImage** – Komentarz nieoznaczony jako spam, ponieważ zawiera tylko obrazy
- **NotSpamIsOnlyReacts** – Komentarz nieoznaczony jako spam, ponieważ zawiera tylko reakcje
- **NotSpamNoLinkOrMention** – Komentarz nieoznaczony jako spam, ponieważ nie zawiera podejrzanych linków ani wzmianek
- **NotSpamPerfectTrustFactor** – Komentarz nieoznaczony jako spam, ze względu na wysokie zaufanie użytkownika
- **NotSpamTooShort** – Komentarz nieoznaczony jako spam, ponieważ jest zbyt krótki do analizy
- **NotSpamSkipped** – Sprawdzenie spamu zostało pominięte
- **NotSpamFromEngine** – Komentarz uznany za nie-spam przez silnik wykrywania (zawiera nazwę silnika i czynnik zaufania)

### Zdarzenia wulgaryzmów/Profanacji
- **BadWordsCheckFailed** – Sprawdzenie filtru wulgaryzmów napotkało błąd
- **BadWordsFoundBadPhrase** – Filtr wulgaryzmów wykrył nieodpowiednie wyrażenie (zawiera wyrażenie)
- **BadWordsFoundBadWord** – Filtr wulgaryzmów wykrył nieodpowiednie słowo (zawiera słowo)
- **BadWordsNoDefinitionForLocale** – Brak definicji wulgaryzmów dostępnych dla języka komentarza (zawiera lokalizację)

### Zdarzenia weryfikacji użytkownika
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** – Komentarz wymaga weryfikacji, ale użytkownik nie jest w zweryfikowanej sesji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** – Komentarz wymaga weryfikacji, ale użytkownik nie został jeszcze zweryfikowany
- **InVerifiedSession** – Użytkownik publikujący komentarz jest w zweryfikowanej sesji
- **SentVerificationEmailNoSession** – Email weryfikacyjny wysłany do niezweryfikowanego użytkownika
- **SentWelcomeEmail** – Email powitalny wysłany do nowego użytkownika

### Zdarzenia zaufania i bezpieczeństwa
- **TrustFactorChanged** – Czynnik zaufania użytkownika został zmodyfikowany (zawiera wartości przed i po)
- **SpamFilterDisabledBecauseAdmin** – Filtrowanie spamu pominięte dla użytkownika admina
- **TenantSpamFilterDisabled** – Filtrowanie spamu wyłączone dla całego najemcy
- **RepeatCommentCheckIgnored** – Sprawdzenie powtarzających się komentarzy pominięte (zawiera powód)
- **UserIsAdmin** – Użytkownik zidentyfikowany jako admin
- **UserIsAdminParentTenant** – Użytkownik zidentyfikowany jako admin nadrzędnego najemcy
- **UserIsAdminViaSSO** – Użytkownik zidentyfikowany jako admin poprzez SSO
- **UserIsMod** – Użytkownik zidentyfikowany jako moderator

### Zmiany statusu komentarza
Zdarzenia zmiany statusu zawierają wartości przed i po, oraz użytkownika, który dokonał zmiany:
- **ExpireStatusChanged** – Status wygaśnięcia komentarza został zmodyfikowany
- **ReviewStatusChanged** – Status przeglądu komentarza został zmieniony
- **SpamStatusChanged** – Status spamu komentarza został zaktualizowany
- **ApproveStatusChanged** – Status zatwierdzenia komentarza został zmieniony
- **TextChanged** – Zawartość tekstowa komentarza została edytowana (zawiera tekst przed i po)
- **VotesChanged** – Liczba głosów komentarza została zaktualizowana (zawiera szczegółowy podział głosów)
- **Flagged** – Komentarz został oznaczony przez użytkowników
- **UnFlagged** – Oznaczenia komentarza zostały usunięte

### Działania moderacyjne
- **Pinned** – Komentarz został przypięty przez moderatora (zawiera, kto go przypiął)
- **UnPinned** – Komentarz został odpięty przez moderatora (zawiera, kto go odpiął)

### Zdarzenia powiadomień
- **CreatedNotifications** – Powiadomienia zostały utworzone dla komentarza (zawiera liczbę powiadomień)
- **NotificationCreateFailure** – Nie udało się utworzyć powiadomień
- **BadgeAwarded** – Odznaka użytkownika została przyznana za komentarz (zawiera nazwę odznaki)

### Zdarzenia powiadomień o wzmiankach i odpowiedziach
Te zdarzenia określają osobę, która otrzymałaby e‑mail lub powiadomienie. Gdy nic nie zostało wysłane, kolumna Szczegóły wyjaśnia przyczynę.
- **MentionEmailSent** – Użytkownik wspomniany w komentarzu otrzymał e‑mail
- **MentionEmailSkipped** – Wspomniany użytkownik nie otrzymał e‑maila (zawiera powód)
- **MentionHeldForApproval** – E‑mail z wzmianką czeka, aż komentarz zostanie zatwierdzony
- **MentionNotificationCreated** – Wspomniany użytkownik otrzymał powiadomienie w aplikacji
- **MentionNotificationSkipped** – Wspomniany użytkownik nie otrzymał powiadomienia w aplikacji (zawiera powód)
- **ReplyEmailSent** – Autor komentarza, na który odpowiadano, otrzymał e‑mail o tej odpowiedzi
- **ReplyEmailSkipped** – Autor komentarza, na który odpowiadano, nie otrzymał e‑maila (zawiera powód)
- **ReplyNotificationSkipped** – Autor komentarza, na który odpowiadano, nie otrzymał powiadomienia w aplikacji (zawiera powód)

Powody wyświetlane, gdy e‑mail lub powiadomienie nie zostało wysłane:
- Użytkownik już nie istnieje lub nie ma adresu e‑mail
- Użytkownik wyłączył powiadomienia e‑mail lub wyłączył powiadomienia dla tego wątku
- Jeden z użytkowników zablokował drugiego
- Użytkownicy nie należą do żadnej tej samej grupy SSO
- Adres e‑mail użytkownika znajduje się na liście tłumionych po odbiciu lub skargi na spam (zobacz [Email Suppression Management](/guide-notifications.html#email-suppression-management))
- Adres e‑mail użytkownika jest w domenie example.com, która nie może otrzymywać e‑maili
- Komentarz został oznaczony jako spam, usunięty lub niezatwierdzony w ciągu 7 dni
- Komentarz, na który odpowiadano, został pozostawiony anonimowo
- Użytkownik odpowiedział na własny komentarz
- Użytkownik został wspomniany w odpowiedzi, więc otrzymał e‑mail z wzmianką zamiast e‑maila z odpowiedzią
- Użytkownik już miał powiadomienie o odpowiedzi na ten komentarz
- Wysyłanie nie powiodło się 5 razy

Jeśli dostawa nie powiedzie się lub osiągnie limit wysyłki, e‑mail jest kolejkuowany do ponownej próby i wpis w logu to odnotowuje.

### Zdarzenia publikacji
- **PublishedLive** – Komentarz został opublikowany dla aktywnych subskrybentów (zawiera liczbę subskrybentów)

### Zdarzenia integracji
- **WebhookSynced** – Komentarz został zsynchronizowany za pośrednictwem webhooka

### Zdarzenia reguł spamu
- **SpamRuleMatch** – Komentarz dopasował się do niestandardowej reguły spamu (zawiera szczegóły reguły)

### Zdarzenia lokalizacji
- **LocaleDetectedFromText** – Lokalizacja językowa została automatycznie wykryta z tekstu komentarza (zawiera wykryty język i lokalizację)

## Przypadki użycia logów komentarzy

Logi komentarzy są automatycznie generowane i przechowywane wraz z każdym komentarzem. Dostarczają cennych informacji dla:
- **Understanding moderation decisions** – Zobacz dokładnie, dlaczego komentarz został zatwierdzony, wstrzymany do przeglądu lub oznaczony jako spam
- **Debugging approval/spam issues** – Śledź logikę decyzji, gdy komentarze nie zachowują się zgodnie z oczekiwaniami
- **Tracking user behavior patterns** – Monitoruj zmiany czynnika zaufania i status weryfikacji
- **Auditing moderator actions** – Przeglądaj, jakie działania podjęli moderatorzy na konkretnych komentarzach
- **Investigating spam filter effectiveness** – Zobacz, które silniki wykrywania łapią spam, a które nie
- **Troubleshooting integrations** – Weryfikuj synchronizacje webhooków i dostarczanie powiadomień

Te logi pomagają utrzymać przejrzystość procesu moderacji i wspierają dopasowywanie zachowania systemu komentarzy.