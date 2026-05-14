#### "Registrierungs-Token nicht gefunden, abgelaufen oder bereits verwendet"

Das Token in Ihrer Registrierungs-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>) ist 30 Minuten gültig und kann nur einmal verwendet werden. Wenn Ihr LMS länger gebraucht hat oder die Registrierung nach einem erfolgreichen Versuch erneut ausgeführt wurde, wird das Token abgelehnt. Generieren Sie eine neue URL auf der FastComments LTI 1.3 Konfigurationsseite und beginnen Sie erneut.

#### "Plattform hat Registrierung abgelehnt"

Ihr LMS hat den Registrierungs-Handshake abgelehnt. Die häufigsten Ursachen:

- **Tool already registered with the same client name.** Einige Plattformen (insbesondere D2L) lehnen eine zweite Registrierung von "FastComments" ab, bis die vorherige gelöscht wurde. Entfernen Sie das alte Tool in Ihrem LMS und versuchen Sie es erneut.
- **Wrong field in the LMS.** Stellen Sie sicher, dass Sie die URL in das **registration / tool initiation registration endpoint**-Feld eingefügt haben, nicht in das launch URL- oder login URL-Feld.
- **The LMS doesn't actually support Dynamic Registration.** Ältere Moodle- und Blackboard-Versionen geben LTI 1.3 an, erlauben jedoch nur manuelle Konfiguration. Prüfen Sie die Dokumentation Ihrer Plattform.

#### "Fehler beim Abrufen der Plattformkonfiguration"

FastComments konnte das openid-configuration-Dokument Ihres LMS nicht lesen. Das ist selten und bedeutet normalerweise, dass das LMS eine fehlerhafte oder nicht erreichbare discovery URL geliefert hat. Kontaktieren Sie den Support Ihres LMS.

#### Start zeigt "Konfiguration nicht gefunden"

Entweder wurde die Konfiguration in FastComments gelöscht, oder der Start kam von einem `iss`/`client_id`-Paar, das wir nicht erkennen. Wenn Sie gelöscht und neu registriert haben, weisen Sie Ihr LMS an, das FastComments-Tool zu entfernen und erneut hinzuzufügen, damit es die neue client_id erhält.

#### Start zeigt "Deployment nicht registriert"

Sie haben FastComments von einer Brightspace-/Moodle-/Blackboard-Bereitstellung gestartet, die sich von derjenigen unterscheidet, in der es zuerst gestartet wurde. FastComments pinnt die `deployment_id` beim ersten Start als Sicherheitsprüfung. Um eine neue Bereitstellung unter demselben Client hinzuzufügen, kontaktieren Sie den Support – wir fügen die deployment ID zur Konfiguration hinzu.

#### Start zeigt "Nicht unterstützter message_type"

Das LMS hat eine LTI-Nachricht gesendet, die FastComments nicht verarbeitet (z. B. `LtiSubmissionReviewRequest`). FastComments unterstützt nur die standardmäßigen resource-link-Launch- und deep-linking-Flows. Melden Sie sich bei uns, wenn Sie einen bestimmten Nachrichtentyp hinzugefügt haben möchten.

#### Iframe passt sich nicht an

Die meisten LMS passen LTI-Iframes automatisch an. Wenn Ihres das nicht tut, prüfen Sie, ob die Starteinstellungen des LMS es dem Tool erlauben, postMessage-Ereignisse an den übergeordneten Frame zu senden. FastComments sendet sowohl Canvas-Style- (`lti.frameResize`) als auch IMS-Spezifikations- (`org.imsglobal.lti.frameResize`) Resize-Nachrichten.

---