FastComments verfolgt automatisch detaillierte Ereignisse für jeden Kommentar, um Transparenz bei Moderationsentscheidungen und Systemaktionen zu gewährleisten. Diese Protokolle helfen Ihnen zu verstehen, warum ein Kommentar genehmigt, als Spam markiert oder sein Status geändert wurde.

Sie können die Kommentarprotokolle für einzelne Kommentare im Moderate Comments-Dashboard einsehen, indem Sie einen bestimmten Kommentar auswählen.

## Comment Log Events

Jeder Kommentar führt ein Protokoll der Ereignisse, die während seines Lebenszyklus auftreten. Nachfolgend sind die Arten von Ereignissen aufgeführt, die erfasst werden:

### Anonymization Events
- **Anonymized** - Kommentarinhalt wurde gelöscht und Benutzer als gelöscht markiert

### Approval Events
- **ApprovedDueToPastComment** - Kommentar genehmigt, weil der Benutzer zuvor genehmigte Kommentare hat
- **ApprovedIsAdmin** - Kommentar genehmigt, weil der Benutzer ein Admin ist
- **NotApprovedRequiresApproval** - Kommentar erfordert manuelle Genehmigung

### Spam Detection Events
- **IsSpam** - Kommentar von der Erkennungs-Engine als Spam markiert
- **IsSpamDueToBadWords** - Kommentar aufgrund des Fluchfilter als Spam markiert
- **IsSpamFromLLM** - Kommentar von der KI/LLM-Engine als Spam markiert
- **IsSpamRepeatComment** - Kommentar als Spam markiert, weil er wiederholend ist
- **NotSpamIsOnlyImage** - Kommentar nicht als Spam markiert, weil er nur Bilder enthält
- **NotSpamIsOnlyReacts** - Kommentar nicht als Spam markiert, weil er nur Reaktionen enthält
- **NotSpamNoLinkOrMention** - Kommentar nicht als Spam markiert, da keine verdächtigen Links oder Erwähnungen vorhanden sind
- **NotSpamPerfectTrustFactor** - Kommentar nicht als Spam markiert aufgrund hoher Vertrauenswürdigkeit des Benutzers
- **NotSpamTooShort** - Kommentar nicht als Spam markiert, da er zu kurz zur Analyse ist
- **NotSpamSkipped** - Spam-Prüfung wurde übersprungen
- **NotSpamFromEngine** - Kommentar von der Erkennungs-Engine als nicht Spam befunden

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Überprüfung des Fluchfilters ist fehlgeschlagen
- **BadWordsFoundBadPhrase** - Fluchfilter hat unangemessene Phrase erkannt
- **BadWordsFoundBadWord** - Fluchfilter hat unangemessenes Wort erkannt
- **BadWordsNoDefinitionForLocale** - Keine Fluchwortdefinitionen für die Kommentarsprache verfügbar

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Kommentar erfordert Verifizierung, aber Benutzer befindet sich nicht in einer verifizierten Sitzung
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Kommentar erfordert Verifizierung, aber Benutzer noch nicht verifiziert
- **InVerifiedSession** - Benutzer, der den Kommentar postet, befindet sich in einer verifizierten Sitzung
- **SentVerificationEmailNoSession** - Verifizierungs-E-Mail an nicht verifizierten Benutzer gesendet
- **SentWelcomeEmail** - Begrüßungs-E-Mail an neuen Benutzer gesendet

### Trust and Security Events
- **TrustFactorChanged** - Vertrauensfaktor des Benutzers wurde geändert
- **SpamFilterDisabledBecauseAdmin** - Spam-Filter für Admin-Benutzer umgangen
- **TenantSpamFilterDisabled** - Spam-Filter für den gesamten Mandanten deaktiviert
- **RepeatCommentCheckIgnored** - Prüfung auf wiederholte Kommentare wurde umgangen
- **UserIsAdmin** - Benutzer als Admin identifiziert
- **UserIsAdminParentTenant** - Benutzer als Admin des übergeordneten Mandanten identifiziert
- **UserIsAdminViaSSO** - Benutzer via SSO als Admin identifiziert
- **UserIsMod** - Benutzer als Moderator identifiziert

### Comment Status Changes
- **ExpireStatusChanged** - Ablaufstatus des Kommentars wurde geändert
- **ReviewStatusChanged** - Überprüfungsstatus des Kommentars wurde geändert
- **SpamStatusChanged** - Spam-Status des Kommentars wurde aktualisiert
- **ApproveStatusChanged** - Genehmigungsstatus des Kommentars wurde geändert
- **TextChanged** - Kommentartext wurde bearbeitet
- **VotesChanged** - Stimmenanzahl des Kommentars wurde aktualisiert
- **Flagged** - Kommentar wurde von Benutzern markiert
- **UnFlagged** - Markierungen des Kommentars wurden entfernt

### Moderation Actions
- **Pinned** - Kommentar wurde vom Moderator angepinnt
- **UnPinned** - Kommentar wurde vom Moderator entpinnt
- **RestoredFromAnonymized** - Kommentar wurde aus dem anonymisierten Zustand wiederhergestellt

### Notification Events
- **CreatedNotifications** - Benachrichtigungen wurden für den Kommentar erstellt
- **NotificationCreateFailure** - Erstellung von Benachrichtigungen fehlgeschlagen
- **BadgeAwarded** - Benutzerabzeichen für den Kommentar verliehen

### Publishing Events
- **PublishedLive** - Kommentar wurde an Live-Abonnenten veröffentlicht

### Integration Events
- **WebhookSynced** - Kommentar wurde über Webhook synchronisiert

### Spam Rule Events
- **SpamRuleMatch** - Kommentar hat mit einer benutzerdefinierten Spam-Regel übereingestimmt

## Accessing Comment Logs

Kommentarprotokolle werden automatisch generiert und mit jedem Kommentar gespeichert. Sie liefern wertvolle Einblicke für:

- Das Verständnis von Moderationsentscheidungen
- Die Fehlersuche bei Genehmigungs-/Spam-Problemen
- Die Nachverfolgung von Benutzerverhaltensmustern
- Die Prüfung von Systemaktionen

Diese Protokolle helfen, Transparenz im Moderationsprozess aufrechtzuerhalten und unterstützen bei der Feinabstimmung des Verhaltens Ihres Kommentarsystems.