FastComments automatycznie śledzi szczegółowe zdarzenia dla każdego komentarza, aby zapewnić przejrzystość decyzji moderacyjnych i działań systemu. Te dzienniki pomagają zrozumieć, dlaczego komentarz został zatwierdzony, oznaczony jako spam lub zmieniono jego status.

## Accessing Comment Logs

Aby wyświetlić dzienniki dla konkretnego komentarza:

1. Przejdź do strony **Moderuj komentarze** w panelu FastComments
2. Znajdź komentarz, który chcesz sprawdzić
3. Kliknij przycisk **Wyświetl dzienniki** (ikona zegara) na pasku akcji komentarza
4. Pojawi się okno dialogowe pokazujące pełną historię zdarzeń dla tego komentarza

Każdy wpis w dzienniku wyświetla:
- **Kiedy** - Znacznik czasu zdarzenia
- **Kto** - Użytkownik lub system, który wywołał zdarzenie (jeśli dotyczy)
- **Co** - Rodzaj akcji lub zdarzenia
- **Szczegóły** - Dodatkowy kontekst, taki jak wartości przed/po, nazwy silników lub powiązane dane

## Comment Log Events

Każdy komentarz prowadzi dziennik zdarzeń występujących w jego cyklu życia. Poniżej znajdują się typy zdarzeń, które są śledzone:

### Anonymization Events
- **Anonymized** - Treść komentarza została wyczyszczona, a użytkownik oznaczony jako usunięty
- **RestoredFromAnonymized** - Komentarz został przywrócony ze stanu zanonimizowanego

### Approval Events
- **ApprovedDueToPastComment** - Komentarz zatwierdzony, ponieważ użytkownik ma wcześniej zatwierdzone komentarze (zawiera odniesienie do poprzedniego komentarza)
- **ApprovedIsAdmin** - Komentarz zatwierdzony, ponieważ użytkownik jest administratorem
- **NotApprovedRequiresApproval** - Komentarz wymaga ręcznego zatwierdzenia
- **NotApprovedLowTrustFactor** - Komentarz nie został zatwierdzony z powodu niskiego współczynnika zaufania użytkownika (zawiera wartość współczynnika zaufania)

### Profile Comment Approval Events

Te zdarzenia dotyczą konkretnie komentarzy na profilach użytkowników:

- **ApprovedProfileAutoApproveAll** - Komentarz na profilu został automatycznie zatwierdzony, ponieważ właściciel profilu włączył automatyczne zatwierdzanie wszystkich komentarzy
- **ApprovedProfileTrusted** - Komentarz na profilu zatwierdzony, ponieważ komentujący jest zaufany (zawiera odniesienie do komentarza, który ustanowił zaufanie)
- **NotApprovedProfileManualApproveAll** - Komentarz na profilu wymaga ręcznego zatwierdzenia, ponieważ właściciel profilu włączył ręczne zatwierdzanie
- **NotApprovedProfileNotTrusted** - Komentarz na profilu nie został zatwierdzony, ponieważ komentujący nie jest zaufany
- **NotApprovedProfileNewUser** - Komentarz na profilu nie został zatwierdzony, ponieważ komentujący jest nowym użytkownikiem

### Spam Detection Events
- **IsSpam** - Komentarz oznaczony jako spam przez silnik wykrywający (zawiera informację, który silnik podjął decyzję)
- **IsSpamDueToBadWords** - Komentarz oznaczony jako spam z powodu filtra wulgaryzmów
- **IsSpamFromLLM** - Komentarz oznaczony jako spam przez silnik AI/LLM (zawiera nazwę silnika, odpowiedź i liczbę tokenów)
- **IsSpamRepeatComment** - Komentarz oznaczony jako spam z powodu powtarzalności (zawiera informację, który silnik to wykrył)
- **NotSpamIsOnlyImage** - Komentarz nie został oznaczony jako spam, ponieważ zawiera tylko obrazy
- **NotSpamIsOnlyReacts** - Komentarz nie został oznaczony jako spam, ponieważ zawiera tylko reakcje
- **NotSpamNoLinkOrMention** - Komentarz nie został oznaczony jako spam z powodu braku podejrzanych linków lub wzmianek
- **NotSpamPerfectTrustFactor** - Komentarz nie został oznaczony jako spam z powodu wysokiego poziomu zaufania użytkownika
- **NotSpamTooShort** - Komentarz nie został oznaczony jako spam, ponieważ jest zbyt krótki, by go przeanalizować
- **NotSpamSkipped** - Sprawdzenie spamu zostało pominięte
- **NotSpamFromEngine** - Komentarz uznany za nie-spam przez silnik wykrywający (zawiera nazwę silnika i współczynnik zaufania)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Sprawdzenie filtra wulgaryzmów zakończyło się błędem
- **BadWordsFoundBadPhrase** - Filtr wulgaryzmów wykrył nieodpowiednie wyrażenie (zawiera to wyrażenie)
- **BadWordsFoundBadWord** - Filtr wulgaryzmów wykrył nieodpowiednie słowo (zawiera to słowo)
- **BadWordsNoDefinitionForLocale** - Brak definicji wulgaryzmów dla języka komentarza (zawiera lokalizację)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentarz wymaga weryfikacji, ale użytkownik nie jest w zweryfikowanej sesji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentarz wymaga weryfikacji, ale użytkownik nie został jeszcze zweryfikowany
- **InVerifiedSession** - Użytkownik publikujący komentarz jest w zweryfikowanej sesji
- **SentVerificationEmailNoSession** - Wysłano e-mail weryfikacyjny do niezwerifikowanego użytkownika
- **SentWelcomeEmail** - Wysłano e-mail powitalny do nowego użytkownika

### Trust and Security Events
- **TrustFactorChanged** - Współczynnik zaufania użytkownika został zmieniony (zawiera wartości przed i po)
- **SpamFilterDisabledBecauseAdmin** - Filtrowanie spamu zostało wyłączone dla użytkownika z uprawnieniami administratora
- **TenantSpamFilterDisabled** - Filtrowanie spamu wyłączone dla całego tenant-a
- **RepeatCommentCheckIgnored** - Sprawdzenie powtarzających się komentarzy zostało pominięte (zawiera powód)
- **UserIsAdmin** - Użytkownik zidentyfikowany jako administrator
- **UserIsAdminParentTenant** - Użytkownik zidentyfikowany jako administrator nadrzędnego tenant-a
- **UserIsAdminViaSSO** - Użytkownik zidentyfikowany jako administrator przez SSO
- **UserIsMod** - Użytkownik zidentyfikowany jako moderator

### Comment Status Changes

Zdarzenia zmiany statusu zawierają wartości przed i po, oraz użytkownika, który wprowadził zmianę:

- **ExpireStatusChanged** - Zmieniono status wygaśnięcia komentarza
- **ReviewStatusChanged** - Zmieniono status przeglądu komentarza
- **SpamStatusChanged** - Zaktualizowano status spamu komentarza
- **ApproveStatusChanged** - Zmieniono status zatwierdzenia komentarza
- **TextChanged** - Treść komentarza została edytowana (zawiera tekst przed i po)
- **VotesChanged** - Zaktualizowano liczniki głosów komentarza (zawiera szczegółowy podział głosów)
- **Flagged** - Komentarz został oznaczony przez użytkowników
- **UnFlagged** - Flagi komentarza zostały usunięte

### Moderation Actions
- **Pinned** - Komentarz został przypięty przez moderatora (zawiera informację, kto go przypiął)
- **UnPinned** - Komentarz został odpięty przez moderatora (zawiera informację, kto go odpiął)

### Notification Events
- **CreatedNotifications** - Dla komentarza utworzono powiadomienia (zawiera liczbę powiadomień)
- **NotificationCreateFailure** - Niepowodzenie przy tworzeniu powiadomień
- **BadgeAwarded** - Odznaka użytkownika została przyznana za komentarz (zawiera nazwę odznaki)

### Publishing Events
- **PublishedLive** - Komentarz został opublikowany dla subskrybentów na żywo (zawiera liczbę subskrybentów)

### Integration Events
- **WebhookSynced** - Komentarz został zsynchronizowany przez webhook

### Spam Rule Events
- **SpamRuleMatch** - Komentarz pasuje do niestandardowej reguły spamu (zawiera szczegóły reguły)

### Localization Events
- **LocaleDetectedFromText** - Język lokalizacji został automatycznie wykryty na podstawie tekstu komentarza (zawiera wykryty język i lokalizację)

## Use Cases for Comment Logs

Dzienniki komentarzy są generowane automatycznie i przechowywane wraz z każdym komentarzem. Dostarczają cennych informacji do:

- **Zrozumienia decyzji moderacyjnych** - Zobacz dokładnie, dlaczego komentarz został zatwierdzony, przekazany do przeglądu lub oznaczony jako spam
- **Debugowania problemów z zatwierdzaniem/spamem** - Prześledź logikę decyzyjną, gdy komentarze nie zachowują się zgodnie z oczekiwaniami
- **Śledzenia wzorców zachowań użytkowników** - Monitoruj zmiany współczynnika zaufania i status weryfikacji
- **Audytu działań moderatorów** - Przejrzyj działania moderatorów podjęte względem konkretnych komentarzy
- **Badania skuteczności filtrów spamu** - Zobacz, które silniki wykrywają spam, a które nie
- **Rozwiązywania problemów z integracjami** - Zweryfikuj synchronizacje webhooków i dostarczanie powiadomień

Te dzienniki pomagają utrzymać przejrzystość w procesie moderacji i pomagają w dopracowywaniu zachowania systemu komentarzy.