## Aktionen und Suchen

Aktionen erstellen Daten in FastComments; Suchen rufen Daten ab, damit ein späterer Schritt sie verwenden kann. Jede Aktion ruft die FastComments REST API auf und verbraucht die gleichen API‑Guthaben, die der Aufruf in Ihrem eigenen Code kosten würde: ein Guthaben pro Aufruf, sofern nicht anders angegeben.

## Kommentar erstellen

Veröffentlicht einen Kommentar auf einer Seite.

| Feld | Erforderlich | Hinweise |
|------|--------------|----------|
| Seiten‑URL‑ID | Ja | Die URL‑ID, die das Kommentar‑Widget auf der Seite verwendet. Kommentare werden danach gruppiert. |
| Seiten‑URL | Ja | Die vollständige Seiten‑URL, die in Benachrichtigungs‑E‑Mails verwendet wird. |
| Kommentar | Ja | Der Kommentartext im FastComments‑Markdown. |
| Name des Kommentators | Ja | Namen sind pro E‑Mail eindeutig, daher schlägt die Wiederverwendung eines Namens mit einer anderen E‑Mail fehl. |
| E‑Mail des Kommentators | Nein | Ein Benutzer wird für die E‑Mail erstellt, wenn er noch nicht existiert. |
| Benutzer‑ID | Nein | Eine vorhandene SSO‑Benutzer‑ID. Hat Vorrang vor Name und E‑Mail. |
| Elternkommentar‑ID | Nein | Festlegen, um eine Antwort zu posten. |
| Genehmigt, Verifiziert | Nein | Beide standardmäßig auf true gesetzt. Nicht genehmigte Kommentare bleiben verborgen, bis sie moderiert werden. |
| Veröffentlicht am | Nein | Standardmäßig jetzt. |
| Avatar‑URL, Seitentitel, Gebietsschema | Nein | Gebietsschema standardmäßig `en_us`. |
| In Widget live anzeigen | Nein | Schiebt den Kommentar in Echtzeit zu den Betrachtern. Kostet 2 Guthaben statt 1. |
| Spam‑Check ausführen, E‑Mails senden | Nein | Standardmäßig deaktiviert. |

## Seite erstellen

Erstellt einen Seiten‑Datensatz, bevor ein Kommentar darauf existiert, damit er aufgelistet und eingeschränkt werden kann. Erfordert die URL‑ID, den Titel, die URL und optional die SSO‑Gruppen‑IDs, die sie sehen dürfen.

## SSO‑Benutzer erstellen

Erstellt einen Single Sign‑On‑Benutzer. Erfordert Ihre eigene Benutzer‑ID, Benutzernamen und E‑Mail sowie optional Anzeigenamen, Anzeigeetikett, Avatar, Website, Gruppen‑IDs und Benachrichtigungs‑ und Datenschutzeinstellungen. Administrative Rollen können nicht über Zapier vergeben werden.

## Feed‑Beitrag erstellen

Erstellt einen Beitrag in einem FastComments‑Feed aus HTML‑Inhalt, mit optionalem Titel, Autor, Tags und einer Link‑Vorschau.

## Hashtag erstellen

Erstellt einen Hashtag, den Kommentatoren verwenden können, mit optionaler URL, auf die er verweist.

## Kommentar melden

Markiert einen Kommentar zur Moderatorenprüfung. Geben Sie die ID des Benutzers an, der die Meldung vornimmt, oder lassen Sie sie leer, um als Zapier‑Integration zu melden.

## Suchen

| Suche | Eingabe | Rückgabe |
|-------|---------|----------|
| Kommentar finden | Kommentar‑ID | Der Kommentar oder nichts. |
| SSO‑Benutzer finden | E‑Mail | Der SSO‑Benutzer oder nichts. |
| Seite finden | URL‑ID | Die Seite oder nichts. |

Eine Suche, die nichts findet, führt nicht zum Fehlschlagen des Zaps. Kombinieren Sie eine Suche mit einer Erstellung im "find or create"-Modus von Zapier, um die Seite oder den Benutzer zu erstellen, wenn sie fehlt.