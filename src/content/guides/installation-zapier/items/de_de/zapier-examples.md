## Beispiel-Zaps

Einige Workflows, die in wenigen Minuten eingerichtet sind.

**Benachrichtigt werden, wenn neue Kommentare eingehen.** New Comment, dann Slack "Send Channel Message" oder Discord "Send
Channel Message". Mappe den Namen des Kommentators, den Kommentartext und die Seiten‑URL in die Nachricht. Füge den
Domain‑Filter hinzu, um pro Seite einen anderen Kanal zu benachrichtigen.

**Führe ein Protokoll jedes Kommentars.** New Comment, dann Google Sheets "Create Spreadsheet Row". Füge Deleted
Comment als zweiten Zap hinzu, der eine Zeile mit der Kommentar‑ID anhängt, sodass das Sheet gleichzeitig als Audit‑Trail dient.

**E‑Mail an den Autor senden, wenn ein Kommentar genehmigt wird.** Updated Comment mit einem Zapier‑Filter, bei dem Approved true ist,
dann Gmail "Send Email". Da Updated Comment bei jeder Änderung ausgelöst wird, sorgt der Filter dafür, dass dieser Zap
nur bei Genehmigungen reagiert.

**Kommentatoren zu deinem CRM oder deiner Mailingliste hinzufügen.** New Comment, dann HubSpot "Create or Update Contact" oder
Mailchimp "Add or Update Subscriber" mit der E‑Mail des Kommentators. Beachte deine Datenschutz‑Richtlinie und lokale Gesetze,
bevor du jemanden zu einer Marketing‑Liste hinzufügst.

**Einen Kommentar aus einem Formular erstellen.** Typeform oder Google Forms "New Response", dann FastComments Create Comment
mit der Seiten‑URL‑ID, die deine Seite für Testimonials verwendet. Lasse Approved deaktiviert, um jeden Kommentar vor dem Anzeigen
zu prüfen.

**Ankündigungen in einen Feed posten.** RSS by Zapier "New Item in Feed", dann Create Feed Post mit dem Titel, Inhalt und Link
des Elements.

**Mitglieder als SSO‑Benutzer bereitstellen.** Memberstack, Memberful oder dein eigener Webhook, dann Find SSO User gefolgt
von Create SSO User im Modus "find or create".

**Gemeldete Kommentare eskalieren.** Updated Comment, gefiltert nach einer Flag‑Anzahl größer als null, dann Trello "Create
Card" oder Linear "Create Issue" mit der Kommentar‑ID und einem Link zur Moderationsseite.

**Seiten veröffentlichen, sobald sie live gehen.** WordPress oder Ghost "New Post", dann Create Page mit der Beitrags‑URL, sodass die
Seite gelistet und eingeschränkt ist, bevor der erste Kommentar erscheint.

**Gelöschte Kommentare archivieren.** Deleted Comment, dann Airtable "Create Record" mit dem vollständigen Kommentar für die
Aufbewahrungspflicht.