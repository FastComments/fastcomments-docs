## Aktionen und Suchen

Aktionen erstellen Daten in FastComments; Suchen rufen Daten ab, damit ein späterer Schritt sie verwenden kann. Jede Aktion ruft die FastComments REST‑API auf und verbraucht die gleichen API‑Credits, die der Aufruf in Ihrem eigenen Code kosten würde: ein Credit pro Aufruf, sofern nicht anders angegeben.

## Kommentar erstellen

Veröffentlicht einen Kommentar auf einer Seite.

| Feld | Erforderlich | Hinweise |
|------|--------------|----------|
| Page URL ID | Ja | Die URL‑ID, die das Kommentar‑Widget auf der Seite verwendet. Kommentare werden danach gruppiert. |
| Page URL | Ja | Die vollständige Seiten‑URL, die in Benachrichtigungs‑E‑Mails verwendet wird. |
| Comment | Ja | Der Kommentartext im FastComments‑Markdown. |
| Commenter Name | Ja | Namen sind pro E‑Mail eindeutig, daher schlägt die Wiederverwendung eines Namens mit einer anderen E‑Mail fehl. |
| Commenter Email | Nein | Ein Benutzer wird für die E‑Mail erstellt, wenn er noch nicht existiert. |
| User ID | Nein | Eine vorhandene SSO‑Benutzer‑ID. Hat Vorrang vor Name und E‑Mail. |
| Parent Comment ID | Nein | Festlegen, um eine Antwort zu posten. |
| Approved, Verified | Nein | Beide sind standardmäßig true. Nicht genehmigte Kommentare bleiben verborgen, bis sie moderiert werden. |
| Posted At | Nein | Standardmäßig jetzt. |
| Avatar URL, Page Title, Locale | Nein | Locale ist standardmäßig `en_us`. |
| Show Live In Widget | Nein | Schiebt den Kommentar in Echtzeit zu den Betrachtern. Kostet 2 Credits statt 1. |
| Run Spam Check, Send Emails | Nein | Standardmäßig deaktiviert. |

## Seite erstellen oder aktualisieren

Erstellt einen Seiten‑Datensatz, bevor ein Kommentar darauf existiert, sodass er aufgelistet und eingeschränkt werden kann. Akzeptiert die URL‑ID, den Titel, die URL und optional die SSO‑Gruppen‑IDs, die die Seite sehen dürfen. Wenn bereits eine Seite mit dieser URL‑ID existiert, wird sie mit den angegebenen Feldern aktualisiert, sodass ein Zap wiederholt für dieselbe Seite ausgeführt werden kann.

## SSO‑Benutzer erstellen oder aktualisieren

Erstellt einen Single‑Sign‑On‑Benutzer. Akzeptiert Ihre eigene Benutzer‑ID, Benutzernamen und E‑Mail sowie optional Anzeigenamen, Anzeige‑Label, Avatar, Website, Gruppen‑IDs und Benachrichtigungs‑ sowie Datenschutz‑Flags. Wenn bereits ein Benutzer mit dieser ID existiert, wird er stattdessen aktualisiert. Administrative Rollen können nicht über Zapier vergeben werden.

## Feed‑Beitrag erstellen

Erstellt einen Beitrag in einem FastComments‑Feed aus HTML‑Inhalt. Die Autor‑Benutzer‑ID ist erforderlich (eine FastComments‑ oder SSO‑Benutzer‑ID); Titel, Tags und eine Link‑Vorschau sind optional.

## Hashtag erstellen oder aktualisieren

Erstellt einen Hashtag, den Kommentatoren verwenden können, mit einer optionalen URL, zu der er verlinkt. Wenn der Tag bereits existiert, wird er stattdessen aktualisiert.

## Kommentar melden

Markiert einen Kommentar zur Moderatoren‑Überprüfung. Die ID des Benutzers, der die Meldung vornimmt, ist erforderlich; die vom Erstellen‑Kommentar zurückgegebene Autor‑ID funktioniert.

## Suchen

| Suche | Eingabe | Rückgabe |
|-------|---------|----------|
| Find Comment | Comment ID | Der Kommentar oder nichts. |
| Find SSO User | Email | Der SSO‑Benutzer oder nichts. |
| Find Page | URL ID | Die Seite oder nichts. |

Eine Suche, die nichts findet, lässt den Zap nicht fehlschlagen. Find SSO User und Find Page bieten Zapier‑Option „erstellen, wenn nicht vorhanden“ an, die das passende Erstellen ausführt, wenn nichts gefunden wird.