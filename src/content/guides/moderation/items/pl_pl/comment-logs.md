FastComments automatycznie śledzi szczegółowe zdarzenia dla każdego komentarza, aby zapewnić przejrzystość decyzji moderacyjnych i działań systemu. Te dzienniki pomagają zrozumieć, dlaczego komentarz został zatwierdzony, oznaczony jako spam lub zmieniono jego status.

Możesz przeglądać dzienniki komentarzy dla poszczególnych komentarzy w panelu Moderate Comments, wybierając konkretny komentarz.

## Comment Log Events

Każdy komentarz posiada dziennik zdarzeń, które występują w trakcie jego cyklu życia. Poniżej znajdują się typy śledzonych zdarzeń:

### Anonymization Events
- **Anonymized** - Treść komentarza została wyczyszczona, a użytkownik oznaczony jako usunięty

### Approval Events
- **ApprovedDueToPastComment** - Komentarz zatwierdzony, ponieważ użytkownik miał wcześniej zatwierdzone komentarze
- **ApprovedIsAdmin** - Komentarz zatwierdzony, ponieważ użytkownik jest administratorem
- **NotApprovedRequiresApproval** - Komentarz wymaga ręcznego zatwierdzenia

### Spam Detection Events
- **IsSpam** - Komentarz oznaczony jako spam przez silnik wykrywający
- **IsSpamDueToBadWords** - Komentarz oznaczony jako spam z powodu filtra wulgaryzmów
- **IsSpamFromLLM** - Komentarz oznaczony jako spam przez silnik AI/LLM
- **IsSpamRepeatComment** - Komentarz oznaczony jako spam z powodu powtarzalności
- **NotSpamIsOnlyImage** - Komentarz nieoznaczony jako spam, ponieważ zawiera tylko obrazy
- **NotSpamIsOnlyReacts** - Komentarz nieoznaczony jako spam, ponieważ zawiera tylko reakcje
- **NotSpamNoLinkOrMention** - Komentarz nieoznaczony jako spam z powodu braku podejrzanych linków lub wzmianek
- **NotSpamPerfectTrustFactor** - Komentarz nieoznaczony jako spam z powodu wysokiego wskaźnika zaufania użytkownika
- **NotSpamTooShort** - Komentarz nieoznaczony jako spam, ponieważ jest zbyt krótki do analizy
- **NotSpamSkipped** - Sprawdzenie spamu zostało pominięte
- **NotSpamFromEngine** - Komentarz uznany za nie-spam przez silnik wykrywający

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Sprawdzenie filtra wulgaryzmów napotkało błąd
- **BadWordsFoundBadPhrase** - Filtr wulgaryzmów wykrył nieodpowiednią frazę
- **BadWordsFoundBadWord** - Filtr wulgaryzmów wykrył nieodpowiednie słowo
- **BadWordsNoDefinitionForLocale** - Brak definicji wulgaryzmów dla języka komentarza

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentarz wymaga weryfikacji, ale użytkownik nie znajduje się w zweryfikowanej sesji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentarz wymaga weryfikacji, ale użytkownik jeszcze nie został zweryfikowany
- **InVerifiedSession** - Użytkownik publikujący komentarz znajduje się w zweryfikowanej sesji
- **SentVerificationEmailNoSession** - Wysłano wiadomość weryfikacyjną do niezweryfikowanego użytkownika
- **SentWelcomeEmail** - Wysłano wiadomość powitalną do nowego użytkownika

### Trust and Security Events
- **TrustFactorChanged** - Wskaźnik zaufania użytkownika został zmieniony
- **SpamFilterDisabledBecauseAdmin** - Filtrowanie spamu pominięte dla użytkownika administratora
- **TenantSpamFilterDisabled** - Filtrowanie spamu wyłączone dla całego najemcy
- **RepeatCommentCheckIgnored** - Sprawdzenie powtarzających się komentarzy zostało pominięte
- **UserIsAdmin** - Użytkownik zidentyfikowany jako administrator
- **UserIsAdminParentTenant** - Użytkownik zidentyfikowany jako administrator nadrzędnego najemcy
- **UserIsAdminViaSSO** - Użytkownik zidentyfikowany jako administrator przez SSO
- **UserIsMod** - Użytkownik zidentyfikowany jako moderator

### Comment Status Changes
- **ExpireStatusChanged** - Status wygaśnięcia komentarza został zmieniony
- **ReviewStatusChanged** - Status recenzji komentarza został zmieniony
- **SpamStatusChanged** - Status spamu komentarza został zaktualizowany
- **ApproveStatusChanged** - Status zatwierdzenia komentarza został zmieniony
- **TextChanged** - Treść komentarza została edytowana
- **VotesChanged** - Liczba głosów dla komentarza została zaktualizowana
- **Flagged** - Komentarz został oznaczony przez użytkowników
- **UnFlagged** - Oznaczenia komentarza zostały usunięte

### Moderation Actions
- **Pinned** - Komentarz został przypięty przez moderatora
- **UnPinned** - Komentarz został odpięty przez moderatora
- **RestoredFromAnonymized** - Komentarz został przywrócony ze stanu zanonimizowanego

### Notification Events
- **CreatedNotifications** - Dla komentarza utworzono powiadomienia
- **NotificationCreateFailure** - Nie udało się utworzyć powiadomień
- **BadgeAwarded** - Użytkownik otrzymał odznakę za komentarz

### Publishing Events
- **PublishedLive** - Komentarz został opublikowany dla subskrybentów na żywo

### Integration Events
- **WebhookSynced** - Komentarz został zsynchronizowany poprzez webhook

### Spam Rule Events
- **SpamRuleMatch** - Komentarz spełnił warunki niestandardowej reguły antyspamowej

## Accessing Comment Logs

Dzienniki komentarzy są automatycznie generowane i przechowywane wraz z każdym komentarzem. Dostarczają cennych informacji do:

- Zrozumienia decyzji moderacyjnych
- Debugowania problemów związanych z zatwierdzaniem/spamem
- Śledzenia wzorców zachowań użytkowników
- Audytu działań systemu

Te dzienniki pomagają zachować przejrzystość procesu moderacji i wspomagają dostrajanie zachowania systemu komentarzy.