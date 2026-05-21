Der **FastComments - Bewertungsübersicht**-Block zeigt eine aggregierte Sternebewertung und eine Aufschlüsselung der Bewertungen für eine Seite. Kombinieren Sie ihn mit dem **FastComments**-Block in Produktvorlagen für das Standard-Bewertungs-Layout: Zusammenfassung oben, Bewertungsformular und Bewertungen darunter.

### Voraussetzung: Ratings & Reviews einrichten

Der Bewertungsübersichts-Block zeigt die Bewertungsfragen an, die Sie für Ihren Shop konfiguriert haben. Richten Sie diese zuerst ein:

1. Öffnen Sie die FastComments-App in Ihrem Shopify-Admin.
2. Klicken Sie auf die Kachel **Ratings & Reviews Helper** (oder öffnen Sie [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) direkt).
3. Fügen Sie die Fragen hinzu, die jede*r Bewertende beantworten soll (Gesamtsternebewertung, "wie war die Passform", usw.).

Ohne konfigurierte Fragen hat der Übersichtsblock nichts zu aggregieren.

### Block hinzufügen

1. Öffnen Sie den Shopify-Theme-Editor.
2. Öffnen Sie die **Produkt**-Vorlage (oder die Seitenvorlage, in der Sie die Zusammenfassung haben möchten).
3. Klicken Sie auf **Block hinzufügen** nahe dem oberen Rand des Seitenabschnitts, über der Stelle, an der sich der **FastComments**-Block befinden wird.
4. Wählen Sie unter **Apps** **FastComments - Reviews Summary** aus.
5. Fügen Sie weiter unten auf derselben Seite einen **FastComments**-Block hinzu, falls Sie noch keinen haben, damit Besucher Bewertungen hinterlassen können.
6. Klicken Sie auf **Speichern**.

### Einstellungen

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Override which FastComments tenant the summary reads from. Leave blank to use the store's automatically-configured tenant. | (blank) |
| Custom URL ID | Override the page identifier the summary aggregates against. Use this when the summary lives on a different page from the FastComments block it reflects. | (auto-detected) |

### Wie die Zusammenfassung den Bewertungen zugeordnet wird

Der Bewertungsübersichts-Block verwendet dieselbe automatische Erkennungslogik wie der **FastComments**-Block:

- Produktvorlage: `shopify-product-{product.id}`
- Vorlage für Blogbeiträge: `shopify-article-{article.id}`
- Andere Vorlagen: der Anfragepfad

Bei einer normalen Produktseite teilen die Zusammenfassung und der Kommentarthread automatisch dieselbe URL-ID, ohne dass eine Konfiguration erforderlich ist.

### Tipps

- Die Zusammenfassung ist schreibgeschützt. Um Bewertungen zu sammeln, benötigen Sie einen **FastComments**-Block auf derselben Seite.
- Wenn Sie die Bewertungsfragen im Ratings & Reviews Helper nach dem Sammeln von Bewertungen ändern, wird die Zusammenfassung anhand des neuen Fragensets neu berechnet.