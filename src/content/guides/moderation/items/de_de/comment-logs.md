FastComments verfolgt automatisch detaillierte Ereignisse für jeden Kommentar, um Transparenz bei Moderationsentscheidungen und Systemaktionen zu gewährleisten. Diese Protokolle helfen Ihnen zu verstehen, warum ein Kommentar genehmigt, als Spam markiert oder sein Status geändert wurde.

## Zugriff auf Kommentarprotokolle

Um die Protokolle für einen bestimmten Kommentar anzuzeigen:

1. Navigieren Sie zur Seite **Moderate Comments** in Ihrem FastComments-Dashboard
2. Suchen Sie den Kommentar, den Sie untersuchen möchten
3. Klicken Sie auf die Schaltfläche **View Logs** (Uhr-Symbol) in der Aktionsleiste des Kommentars
4. Ein Dialogfeld wird angezeigt, das die vollständige Verlaufshistorie der Ereignisse für diesen Kommentar zeigt

Jeder Protokolleintrag zeigt an:
- **When** - Der Zeitstempel des Ereignisses
- **Who** - Der Benutzer oder das System, das das Ereignis ausgelöst hat (falls zutreffend)
- **What** - Die Art der Aktion oder des Ereignisses
- **Details** - Zusätzliche Kontextinformationen wie Vorher-/Nachher-Werte, Engine-Namen oder zugehörige Daten

## Kommentarprotokoll-Ereignisse

Jeder Kommentar führt ein Protokoll der Ereignisse, die während seines Lebenszyklus auftreten. Nachfolgend sind die erfassten Ereignistypen aufgeführt:

### Anonymization Events
- **Anonymized** - Der Kommentarinhalt wurde geleert und der Benutzer als gelöscht markiert
- **RestoredFromAnonymized** - Der Kommentar wurde aus dem anonymisierten Zustand wiederhergestellt

### Approval Events
- **ApprovedDueToPastComment** - Kommentar genehmigt, weil der Benutzer zuvor genehmigte Kommentare hat (inkl. Verweis auf den früheren Kommentar)
- **ApprovedIsAdmin** - Kommentar genehmigt, weil der Benutzer ein Administrator ist
- **NotApprovedRequiresApproval** - Kommentar erfordert manuelle Genehmigung
- **NotApprovedLowTrustFactor** - Kommentar nicht genehmigt aufgrund eines niedrigen Vertrauensfaktors des Benutzers (inkl. des Vertrauensfaktorwertes)

### Profile Comment Approval Events

Diese Ereignisse gelten speziell für Kommentare in Benutzerprofilen:

- **ApprovedProfileAutoApproveAll** - Profilkommentar automatisch genehmigt, weil der Profilinhaber die automatische Genehmigung für alle Kommentare aktiviert hat
- **ApprovedProfileTrusted** - Profilkommentar genehmigt, weil der Kommentator vertrauenswürdig ist (inkl. Verweis auf den Kommentar, der das Vertrauen begründet hat)
- **NotApprovedProfileManualApproveAll** - Profilkommentar erfordert manuelle Genehmigung, weil der Profilinhaber die manuelle Genehmigung aktiviert hat
- **NotApprovedProfileNotTrusted** - Profilkommentar nicht genehmigt, weil der Kommentator nicht vertrauenswürdig ist
- **NotApprovedProfileNewUser** - Profilkommentar nicht genehmigt, weil der Kommentator ein neuer Benutzer ist

### Spam Detection Events
- **IsSpam** - Kommentar von einer Erkennungs-Engine als Spam markiert (inkl. Angabe, welche Engine die Entscheidung getroffen hat)
- **IsSpamDueToBadWords** - Kommentar aufgrund des Filters für beleidigende Wörter als Spam markiert
- **IsSpamFromLLM** - Kommentar von einer AI/LLM-Engine als Spam markiert (inkl. Engine-Name, Antwort und Token-Anzahl)
- **IsSpamRepeatComment** - Kommentar als Spam markiert wegen Wiederholung (inkl. Angabe, welche Engine es erkannt hat)
- **NotSpamIsOnlyImage** - Kommentar nicht als Spam markiert, weil er nur Bilder enthält
- **NotSpamIsOnlyReacts** - Kommentar nicht als Spam markiert, weil er nur Reaktionen enthält
- **NotSpamNoLinkOrMention** - Kommentar nicht als Spam markiert, da keine verdächtigen Links oder Erwähnungen vorhanden sind
- **NotSpamPerfectTrustFactor** - Kommentar nicht als Spam markiert aufgrund eines hohen Benutzervertrauensfaktors
- **NotSpamTooShort** - Kommentar nicht als Spam markiert, weil er zu kurz zur Analyse ist
- **NotSpamSkipped** - Spamprüfung wurde übersprungen
- **NotSpamFromEngine** - Kommentar von einer Erkennungs-Engine als kein Spam eingestuft (inkl. Engine-Name und Vertrauensfaktor)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Überprüfung durch den Filter für beleidigende Wörter ist fehlgeschlagen
- **BadWordsFoundBadPhrase** - Filter für beleidigende Wörter hat eine unangemessene Phrase erkannt (inkl. der Phrase)
- **BadWordsFoundBadWord** - Filter für beleidigende Wörter hat ein unangemessenes Wort erkannt (inkl. des Wortes)
- **BadWordsNoDefinitionForLocale** - Für die Sprache des Kommentars sind keine Definitionen für beleidigende Wörter verfügbar (inkl. Locale)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Kommentar erfordert Verifizierung, aber der Benutzer ist nicht in einer verifizierten Sitzung
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Kommentar erfordert Verifizierung, aber der Benutzer ist noch nicht verifiziert
- **InVerifiedSession** - Benutzer, der den Kommentar postet, befindet sich in einer verifizierten Sitzung
- **SentVerificationEmailNoSession** - Verifizierungs-E-Mail an einen nicht verifizierten Benutzer gesendet
- **SentWelcomeEmail** - Willkommens-E-Mail an neuen Benutzer gesendet

### Trust and Security Events
- **TrustFactorChanged** - Der Vertrauensfaktor des Benutzers wurde geändert (inkl. Vorher- und Nachher-Werten)
- **SpamFilterDisabledBecauseAdmin** - Spamfilter wurde für einen Administrator umgangen
- **TenantSpamFilterDisabled** - Spamfilter für den gesamten Mandanten deaktiviert
- **RepeatCommentCheckIgnored** - Prüfung auf wiederholte Kommentare wurde umgangen (inkl. Grund)
- **UserIsAdmin** - Benutzer als Administrator identifiziert
- **UserIsAdminParentTenant** - Benutzer als Administrator des übergeordneten Mandanten identifiziert
- **UserIsAdminViaSSO** - Benutzer als Administrator via SSO identifiziert
- **UserIsMod** - Benutzer als Moderator identifiziert

### Comment Status Changes

Statusänderungsereignisse enthalten Vorher- und Nachher-Werte sowie den Benutzer, der die Änderung vorgenommen hat:

- **ExpireStatusChanged** - Ablaufstatus des Kommentars wurde geändert
- **ReviewStatusChanged** - Prüfungsstatus des Kommentars wurde geändert
- **SpamStatusChanged** - Spam-Status des Kommentars wurde aktualisiert
- **ApproveStatusChanged** - Genehmigungsstatus des Kommentars wurde geändert
- **TextChanged** - Textinhalt des Kommentars wurde bearbeitet (inkl. Vorher- und Nachher-Text)
- **VotesChanged** - Stimmenzahlen des Kommentars wurden aktualisiert (inkl. detaillierter Aufschlüsselung)
- **Flagged** - Kommentar wurde von Benutzern gemeldet
- **UnFlagged** - Meldungen des Kommentars wurden entfernt

### Moderation Actions
- **Pinned** - Kommentar wurde vom Moderator angepinnt (inkl. wer ihn angepinnt hat)
- **UnPinned** - Kommentar wurde vom Moderator entpinnt (inkl. wer ihn entpinnt hat)

### Notification Events
- **CreatedNotifications** - Benachrichtigungen für den Kommentar wurden erstellt (inkl. Anzahl der Benachrichtigungen)
- **NotificationCreateFailure** - Erstellen von Benachrichtigungen fehlgeschlagen
- **BadgeAwarded** - Dem Benutzer wurde für den Kommentar ein Abzeichen verliehen (inkl. Name des Abzeichens)

### Publishing Events
- **PublishedLive** - Kommentar wurde an Live-Abonnenten veröffentlicht (inkl. Anzahl der Abonnenten)

### Integration Events
- **WebhookSynced** - Kommentar wurde per Webhook synchronisiert

### Spam Rule Events
- **SpamRuleMatch** - Kommentar entsprach einer benutzerdefinierten Spam-Regel (inkl. Regel-Details)

### Localization Events
- **LocaleDetectedFromText** - Die Sprach-Locale wurde automatisch aus dem Kommentartext erkannt (inkl. erkannter Sprache und Locale)

## Anwendungsfälle für Kommentarprotokolle

Kommentarprotokolle werden automatisch generiert und mit jedem Kommentar gespeichert. Sie bieten wertvolle Einblicke für:

- **Verständnis von Moderationsentscheidungen** - Sehen Sie genau, warum ein Kommentar genehmigt, zur Überprüfung zurückgehalten oder als Spam markiert wurde
- **Fehlerbehebung bei Genehmigungs-/Spam-Problemen** - Verfolgen Sie die Entscheidungslogik, wenn Kommentare sich ungewohnt verhalten
- **Überwachung von Nutzerverhaltensmustern** - Nachverfolgung von Änderungen des Vertrauensfaktors und Verifizierungsstatus
- **Prüfung von Moderatoraktionen** - Überprüfen Sie, welche Maßnahmen Moderatoren an bestimmten Kommentaren vorgenommen haben
- **Untersuchung der Wirksamkeit des Spamfilters** - Sehen Sie, welche Erkennungs-Engines Spam erfassen und welche nicht
- **Fehlerbehebung bei Integrationen** - Überprüfen Sie Webhook-Synchronisationen und Zustellungen von Benachrichtigungen

Diese Protokolle helfen, Transparenz im Moderationsprozess zu wahren und unterstützen bei der Feinabstimmung des Verhaltens Ihres Kommentar­systems.