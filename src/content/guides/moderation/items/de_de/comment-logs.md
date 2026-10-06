FastComments verfolgt automatisch detaillierte Ereignisse für jeden Kommentar, um Transparenz bei Moderationsentscheidungen und Systemaktionen zu bieten. Diese Protokolle helfen Ihnen zu verstehen, warum ein Kommentar genehmigt, als Spam markiert oder sein Status geändert wurde.

## Zugriff auf Kommentarprotokolle

Um die Protokolle für einen bestimmten Kommentar anzuzeigen:

1. Navigieren Sie zur Seite **Moderate Comments** in Ihrem FastComments-Dashboard
2. Finden Sie den Kommentar, den Sie untersuchen möchten
3. Klicken Sie auf die Schaltfläche **View Logs** (Uhrsymbol) in der Aktionsleiste des Kommentars
4. Ein Dialog wird angezeigt, der die vollständige Historie der Ereignisse für diesen Kommentar zeigt

Jeder Protokolleintrag zeigt:
- **When** – Der Zeitstempel des Ereignisses
- **Who** – Der Benutzer oder das System, das das Ereignis ausgelöst hat (falls zutreffend)
- **What** – Der Typ der Aktion oder des Ereignisses
- **Details** – Zusätzlicher Kontext wie Vorher/Nachher-Werte, Engine-Namen oder verwandte Daten

## Kommentarprotokoll-Ereignisse

Jeder Kommentar führt ein Protokoll der Ereignisse, die während seines Lebenszyklus auftreten. Nachfolgend sind die Arten von Ereignissen aufgeführt, die verfolgt werden:

### Anonymisierungsereignisse
- **Anonymized** – Kommentarinhalt wurde gelöscht und der Benutzer als gelöscht markiert
- **RestoredFromAnonymized** – Kommentar wurde aus dem anonymisierten Zustand wiederhergestellt

### Genehmigungsereignisse
- **ApprovedDueToPastComment** – Kommentar wurde genehmigt, weil der Benutzer zuvor Kommentare genehmigt hat (enthält Verweis auf den früheren Kommentar)
- **ApprovedIsAdmin** – Kommentar wurde genehmigt, weil der Benutzer ein Administrator ist
- **NotApprovedRequiresApproval** – Kommentar erfordert manuelle Genehmigung
- **NotApprovedLowTrustFactor** – Kommentar wurde nicht genehmigt aufgrund eines niedrigen Vertrauensfaktors des Benutzers (enthält den Vertrauensfaktorwert)

### Profilkommentar-Genehmigungsereignisse

Diese Ereignisse gelten speziell für Kommentare auf Benutzerprofilen:
- **ApprovedProfileAutoApproveAll** – Profilkommentar wurde automatisch genehmigt, weil der Profilinhaber die automatische Genehmigung für alle Kommentare aktiviert hat
- **ApprovedProfileTrusted** – Profilkommentar wurde genehmigt, weil der Kommentator vertrauenswürdig ist (enthält Verweis auf den Kommentar, der das Vertrauen begründet hat)
- **NotApprovedProfileManualApproveAll** – Profilkommentar erfordert manuelle Genehmigung, weil der Profilinhaber manuelle Genehmigung aktiviert hat
- **NotApprovedProfileNotTrusted** – Profilkommentar wurde nicht genehmigt, weil der Kommentator nicht vertrauenswürdig ist
- **NotApprovedProfileNewUser** – Profilkommentar wurde nicht genehmigt, weil der Kommentator ein neuer Benutzer ist

### Spam-Erkennungsereignisse
- **IsSpam** – Kommentar wurde von einer Erkennungs-Engine als Spam markiert (enthält, welche Engine die Entscheidung getroffen hat)
- **IsSpamDueToBadWords** – Kommentar wurde aufgrund des Fluchwortfilters als Spam markiert
- **IsSpamFromLLM** – Kommentar wurde von einer KI/LLM-Engine als Spam markiert (enthält Engine-Name, Antwort und Token-Anzahl)
- **IsSpamRepeatComment** – Kommentar wurde wegen Wiederholung als Spam markiert (enthält, welche Engine ihn erkannt hat)
- **NotSpamIsOnlyImage** – Kommentar wurde nicht als Spam markiert, weil er nur Bilder enthält
- **NotSpamIsOnlyReacts** – Kommentar wurde nicht als Spam markiert, weil er nur Reaktionen enthält
- **NotSpamNoLinkOrMention** – Kommentar wurde nicht als Spam markiert, weil keine verdächtigen Links oder Erwähnungen vorhanden sind
- **NotSpamPerfectTrustFactor** – Kommentar wurde nicht als Spam markiert, weil das Vertrauen des Benutzers hoch ist
- **NotSpamTooShort** – Kommentar wurde nicht als Spam markiert, weil er zu kurz zum Analysieren ist
- **NotSpamSkipped** – Spam-Überprüfung wurde übersprungen
- **NotSpamFromEngine** – Kommentar wurde von einer Erkennungs-Engine als kein Spam bestimmt (enthält Engine-Name und Vertrauensfaktor)

### Fluchwort-/Profanitätsereignisse
- **BadWordsCheckFailed** – Profanitätsfilterprüfung stieß auf einen Fehler
- **BadWordsFoundBadPhrase** – Profanitätsfilter erkannte unangemessene Phrase (enthält die Phrase)
- **BadWordsFoundBadWord** – Profanitätsfilter erkannte unangemessenes Wort (enthält das Wort)
- **BadWordsNoDefinitionForLocale** – Keine Profanitätsdefinitionen für die Kommentarsprache verfügbar (enthält das Gebietsschema)

### Benutzerverifizierungsereignisse
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** – Kommentar erfordert Verifizierung, aber Benutzer befindet sich nicht in einer verifizierten Sitzung
- **CommentMustBeVerifiedToApproveNotVerifiedYet** – Kommentar erfordert Verifizierung, aber Benutzer ist noch nicht verifiziert
- **InVerifiedSession** – Benutzer, der den Kommentar postet, befindet sich in einer verifizierten Sitzung
- **SentVerificationEmailNoSession** – Verifizierungs-E-Mail an nicht verifizierten Benutzer gesendet
- **SentWelcomeEmail** – Willkommens-E-Mail an neuen Benutzer gesendet

### Vertrauens- und Sicherheitsereignisse
- **TrustFactorChanged** – Vertrauensfaktor des Benutzers wurde geändert (enthält Vorher- und Nachher-Werte)
- **SpamFilterDisabledBecauseAdmin** – Spam-Filter wurde für Administratorbenutzer umgangen
- **TenantSpamFilterDisabled** – Spam-Filter für den gesamten Mandanten deaktiviert
- **RepeatCommentCheckIgnored** – Wiederholungs-Check wurde umgangen (enthält den Grund)
- **UserIsAdmin** – Benutzer als Administrator identifiziert
- **UserIsAdminParentTenant** – Benutzer als Administrator des übergeordneten Mandanten identifiziert
- **UserIsAdminViaSSO** – Benutzer als Administrator über SSO identifiziert
- **UserIsMod** – Benutzer als Moderator identifiziert

### Kommentarstatusänderungen

Statusänderungsereignisse enthalten Vorher- und Nachher-Werte sowie den Benutzer, der die Änderung vorgenommen hat:
- **ExpireStatusChanged** – Ablaufstatus des Kommentars wurde geändert
- **ReviewStatusChanged** – Prüfungsstatus des Kommentars wurde geändert
- **SpamStatusChanged** – Spam-Status des Kommentars wurde aktualisiert
- **ApproveStatusChanged** – Genehmigungsstatus des Kommentars wurde geändert
- **TextChanged** – Kommentartext wurde bearbeitet (enthält Vorher- und Nachher-Text)
- **VotesChanged** – Abstimmungszahlen des Kommentars wurden aktualisiert (enthält detaillierte Aufschlüsselung der Stimmen)
- **Flagged** – Kommentar wurde von Benutzern gemeldet
- **UnFlagged** – Kommentar-Meldungen wurden entfernt

### Moderationsaktionen
- **Pinned** – Kommentar wurde vom Moderator angeheftet (enthält, wer ihn angeheftet hat)
- **UnPinned** – Kommentar wurde vom Moderator losgelöst (enthält, wer ihn losgelöst hat)

### Benachrichtigungsereignisse
- **CreatedNotifications** – Benachrichtigungen wurden für den Kommentar erstellt (enthält Benachrichtigungsanzahl)
- **NotificationCreateFailure** – Erstellung von Benachrichtigungen fehlgeschlagen
- **BadgeAwarded** – Benutzerabzeichen wurde für den Kommentar vergeben (enthält Abzeichenname)

### Erwähnungs- und Antwort-Benachrichtigungsereignisse

Diese Ereignisse benennen die Person, die die E‑Mail oder Benachrichtigung erhalten würde. Wenn nichts gesendet wurde, gibt die Spalte Details den Grund an.
- **MentionEmailSent** – Ein im Kommentar erwähnter Benutzer erhielt eine E‑Mail
- **MentionEmailSkipped** – Ein erwähnter Benutzer erhielt keine E‑Mail (enthält den Grund)
- **MentionHeldForApproval** – Die Erwähnungs‑E‑Mail wartet, bis der Kommentar genehmigt wird
- **MentionNotificationCreated** – Ein erwähnter Benutzer erhielt eine In‑App‑Benachrichtigung
- **MentionNotificationSkipped** – Ein erwähnter Benutzer erhielt keine In‑App‑Benachrichtigung (enthält den Grund)
- **ReplyEmailSent** – Der Autor des beantworteten Kommentars erhielt eine E‑Mail über diese Antwort
- **ReplyEmailSkipped** – Der Autor des beantworteten Kommentars erhielt keine E‑Mail (enthält den Grund)
- **ReplyNotificationSkipped** – Der Autor des beantworteten Kommentars erhielt keine In‑App‑Benachrichtigung (enthält den Grund)

Gründe, die angezeigt werden, wenn eine E‑Mail oder Benachrichtigung nicht gesendet wurde:
- Der Benutzer existiert nicht mehr oder hat keine E‑Mail‑Adresse
- Der Benutzer hat E‑Mail‑Benachrichtigungen deaktiviert oder Benachrichtigungen für diesen Thread ausgeschaltet
- Einer der beiden Benutzer hat den anderen blockiert
- Die Benutzer sind in keiner gemeinsamen SSO‑Gruppe
- Die E‑Mail‑Adresse des Benutzers steht nach einem Bounce oder einer Spam‑Beschwerde auf der Unterdrückungsliste (siehe [Email Suppression Management](/guide-notifications.html#email-suppression-management))
- Die E‑Mail‑Adresse des Benutzers lautet example.com, was keine E‑Mails empfangen kann
- Der Kommentar wurde innerhalb von 7 Tagen als Spam markiert, gelöscht oder nicht genehmigt
- Der beantwortete Kommentar wurde anonym hinterlassen
- Der Benutzer hat auf seinen eigenen Kommentar geantwortet
- Der Benutzer wurde in der Antwort erwähnt und erhielt daher die Erwähnungs‑E‑Mail statt einer Antwort‑E‑Mail
- Der Benutzer hatte bereits eine Antwort‑Benachrichtigung für den Kommentar
- Versand ist 5‑mal fehlgeschlagen

Wenn die Zustellung fehlschlägt oder ein Sendelimit erreicht wird, wird die E‑Mail in die Warteschlange gestellt, um es erneut zu versuchen, und der Protokolleintrag gibt dies an.

### Veröffentlichungsereignisse
- **PublishedLive** – Kommentar wurde an Live‑Abonnenten veröffentlicht (enthält Abonnentenzahl)

### Integrationsereignisse
- **WebhookSynced** – Kommentar wurde über Webhook synchronisiert

### Spam-Regel-Ereignisse
- **SpamRuleMatch** – Kommentar entsprach einer benutzerdefinierten Spam‑Regel (enthält Regeldetails)

### Lokalisierungsereignisse
- **LocaleDetectedFromText** – Sprachlocale wurde automatisch aus dem Kommentartext erkannt (enthält erkannte Sprache und Locale)

## Anwendungsfälle für Kommentarprotokolle

Kommentarprotokolle werden automatisch erstellt und mit jedem Kommentar gespeichert. Sie bieten wertvolle Einblicke für:
- **Understanding moderation decisions** – Sehen Sie genau, warum ein Kommentar genehmigt, zur Überprüfung zurückgehalten oder als Spam markiert wurde
- **Debugging approval/spam issues** – Verfolgen Sie die Entscheidungslogik, wenn Kommentare nicht wie erwartet funktionieren
- **Tracking user behavior patterns** – Überwachen Sie Änderungen des Vertrauensfaktors und den Verifizierungsstatus
- **Auditing moderator actions** – Überprüfen Sie, welche Aktionen Moderatoren bei bestimmten Kommentaren durchgeführt haben
- **Investigating spam filter effectiveness** – Sehen Sie, welche Erkennungs‑Engines Spam erfassen und welche nicht
- **Troubleshooting integrations** – Überprüfen Sie Webhook‑Synchronisationen und die Zustellung von Benachrichtigungen

Diese Protokolle helfen, Transparenz im Moderationsprozess zu wahren und unterstützen bei der Feinabstimmung des Verhaltens Ihres Kommentar‑Systems.