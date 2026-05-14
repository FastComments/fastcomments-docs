#### "Registrierungs-Token nicht gefunden, abgelaufen oder bereits verwendet"

Das Token in Ihrer Registrierungs-URL ist 30 Minuten lang gültig und kann nur einmal verwendet werden. Wenn Ihr LMS länger gebraucht hat, oder wenn die Registrierung nach einem erfolgreichen Versuch erneut ausgeführt wurde, wird das Token abgelehnt. Erzeugen Sie eine neue URL auf der FastComments LTI 1.3-Konfigurationsseite und beginnen Sie von vorn.

#### "Plattform hat Registrierung abgelehnt"

Ihr LMS hat den Registrierungs-Handshake abgelehnt. Die häufigsten Ursachen:

- **Tool bereits mit demselben Client-Namen registriert.** Einige Plattformen (insbesondere D2L) lehnen eine zweite Registrierung von "FastComments" ab, bis die vorherige gelöscht wurde. Entfernen Sie das alte Tool in Ihrem LMS und versuchen Sie es erneut.
- **Falsches Feld im LMS.** Vergewissern Sie sich, dass Sie die URL in das **registration / tool initiation registration endpoint** Feld eingefügt haben, und nicht in das Feld für die launch URL oder login URL.
- **Das LMS unterstützt Dynamic Registration tatsächlich nicht.** Ältere Moodle- und Blackboard-Versionen geben LTI 1.3 an, erlauben aber nur manuelle Konfiguration. Prüfen Sie die Dokumentation Ihrer Plattform.

#### "Fehler beim Abrufen der Plattformkonfiguration"

FastComments konnte das openid-configuration-Dokument Ihres LMS nicht lesen. Das ist selten und bedeutet normalerweise, dass das LMS eine fehlerhaft formatierte oder nicht erreichbare discovery URL bereitgestellt hat. Kontaktieren Sie den Support Ihres LMS.

#### Launch shows "Configuration not found"

Entweder wurde die Konfiguration in FastComments gelöscht, oder der Startvorgang kam von einem `iss`/`client_id`-Paar, das wir nicht erkennen. Wenn Sie die Konfiguration gelöscht und neu registriert haben, weisen Sie Ihr LMS an, das FastComments-Tool zu entfernen und erneut hinzuzufügen, damit es die neue client_id erhält.

#### Launch shows "Deployment not registered"

Sie haben FastComments von einer Brightspace/Moodle/Blackboard-Bereitstellung gestartet, die sich von derjenigen unterscheidet, in der es zuerst gestartet wurde. FastComments legt die `deployment_id` beim ersten Start als Sicherheitsprüfung fest. Um eine neue Bereitstellung unter demselben Client hinzuzufügen, kontaktieren Sie den Support - wir fügen die deployment ID zur Konfiguration hinzu.

#### Launch shows "Unsupported message_type"

Das LMS hat eine LTI-Nachricht gesendet, die FastComments nicht verarbeitet (z. B. `LtiSubmissionReviewRequest`). FastComments unterstützt nur den standardmäßigen resource-link-Launch und Deep-Linking-Flow. Kontaktieren Sie uns, falls Sie einen bestimmten Nachrichtentyp hinzugefügt haben möchten.

#### Iframe doesn't resize

Die meisten LMS passen LTI-Iframes automatisch an. Wenn Ihres das nicht tut, prüfen Sie, ob die Starteinstellungen des LMS dem Tool erlauben, postMessage-Ereignisse an den übergeordneten Frame zu senden. FastComments sendet sowohl Canvas-Style- (`lti.frameResize`) als auch IMS-Spec- (`org.imsglobal.lti.frameResize`) Resize-Nachrichten.