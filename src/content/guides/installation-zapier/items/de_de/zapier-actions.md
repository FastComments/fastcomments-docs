## Aktionen und Suchen

Aktionen erstellen Daten in FastComments; Suchen rufen Daten ab, damit ein späterer Schritt sie verwenden kann. Jede Aktion ruft die FastComments REST‑API auf und verbraucht die gleichen API‑Credits, die der Aufruf in Ihrem eigenen Code kosten würde: ein Credit pro Aufruf, sofern nicht anders angegeben.

## Kommentar erstellen

Veröffentlicht einen Kommentar auf einer Seite.

| Feld | Erforderlich | Hinweis |
|------|--------------|---------|
| Page URL ID | Yes | Die URL‑ID, die das Kommentar‑Widget auf der Seite verwendet. Kommentare werden danach gruppiert. |
| Page URL | Yes | Die vollständige Seiten‑URL, die in Benachrichtigungs‑E‑Mails verwendet wird. |
| Comment | Yes | Der Kommentartext im FastComments‑Markdown. |
| Commenter Name | Yes | Namen sind pro E‑Mail eindeutig, daher schlägt die Wiederverwendung eines Namens mit einer anderen E‑Mail fehl. |
| Commenter Email | No | Ein Benutzer wird für die E‑Mail erstellt, wenn er noch nicht existiert. |
| User ID | No | Eine vorhandene SSO‑Benutzer‑ID. Hat Vorrang vor Name und E‑Mail. |
| Parent Comment ID | No | Festlegen, um eine Antwort zu posten. |
| Approved, Verified No | Both default to true. Unapproved comments stay hidden until moderated. | Beide standardmäßig auf true gesetzt. Nicht genehmigte Kommentare bleiben verborgen, bis sie moderiert werden. |
| Posted At No | Defaults to now. | Standardmäßig jetzt. |
| Avatar URL, Page Title, Locale No | Locale defaults to `en_us`. | Locale standardmäßig `en_us`. |
| Show Live In Widget No | Pushes the comment to viewers in real time. Costs 2 credits instead of 1. | Schiebt den Kommentar in Echtzeit zu den Betrachtern. Kostet 2 Credits statt 1. |
| Run Spam Check, Send Emails No | Off by default. | Standardmäßig deaktiviert. |

## Seite erstellen

Erstellt einen Seiten‑Datensatz, bevor ein Kommentar darauf existiert, sodass er aufgelistet und eingeschränkt werden kann. Erfordert die URL‑ID, den Titel, die URL und optional die SSO‑Gruppen‑IDs, die sie sehen dürfen.

## SSO‑Benutzer erstellen

Erstellt einen Single‑Sign‑On‑Benutzer. Erfordert Ihre eigene Benutzer‑ID, Benutzernamen und E‑Mail sowie optional Anzeigenamen, Anzeige‑Label, Avatar, Website, Gruppen‑IDs und Benachrichtigungs‑ und Datenschutz‑Flags. Administrative Rollen können nicht über Zapier vergeben werden.

## Feed‑Beitrag erstellen

Erstellt einen Beitrag in einem FastComments‑Feed aus HTML‑Inhalt. Die Benutzer‑ID des Autors ist erforderlich (eine FastComments‑ oder SSO‑Benutzer‑ID); Titel, Tags und eine Link‑Vorschau sind optional.

## Hashtag erstellen

Erstellt einen Hashtag, den Kommentatoren verwenden können, mit einer optionalen URL, auf die er verweist. Tags sind pro Konto eindeutig, sodass ein Zap, der bei jedem Durchlauf einen erstellt, etwas Einzigartiges im Tag benötigen muss.

## Kommentar melden

Markiert einen Kommentar zur Moderatoren‑Überprüfung. Die ID des Benutzers, der die Meldung vornimmt, ist erforderlich; die vom Erstellen‑Kommentar zurückgegebene Autor‑ID funktioniert.

## Suchen

| Suche | Eingabe | Rückgabe |
|-------|---------|----------|
| Find Comment | Comment ID | Der Kommentar oder nichts. |
| Find SSO User | Email | Der SSO‑Benutzer oder nichts. |
| 
| Find Page | URL ID | Die Seite oder nichts. |

Eine Suche, die nichts findet, führt nicht zum Fehlschlagen des Zaps. Kombinieren Sie eine Suche mit einer Erstellung im „find or create“-Modus von Zapier, um die Seite oder den Benutzer zu erstellen, wenn sie fehlt.