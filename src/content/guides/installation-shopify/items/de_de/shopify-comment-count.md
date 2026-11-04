Der Block **FastComments - Comment Count** rendert eine kleine Kommentaranzahl für eine einzelne Seite. Verwenden Sie ihn in Blogartikel-Listen, Produktkarten oder jedem Template, das auf eine Seite mit Kommentaren verlinkt, damit Besucher sehen können, wie aktiv jeder Thread ist, bevor sie weiterklicken.

### Block hinzufügen

1. Öffnen Sie den Shopify-Theme-Editor.
2. Öffnen Sie das Template, in dem die Anzahl erscheinen soll. Zum Beispiel das **Blog**-Template (die Artikelliste) oder einen Produktlisten-Abschnitt.
3. Klicken Sie im Abschnitt, der jedes Element rendert, auf **Block hinzufügen**.
4. Wählen Sie unter **Apps** die Option **FastComments - Comment Count**.
5. Klicken Sie auf **Speichern**.

### Einstellungen

| Einstellung | Funktion | Standard |
|---|---|---|
| Tenant ID (optional) | Überschreiben, von welchem FastComments-Tenant die Anzahl gelesen wird. Lassen Sie das Feld leer, um den automatisch konfigurierten Tenant des Shops zu verwenden. | (leer) |
| Custom URL ID | Überschreiben Sie den Seitenbezeichner, nach dem die Anzahl sucht. Verwenden Sie dies, wenn sich die Anzahl auf einer anderen Seite befindet als der FastComments-Block, den sie verfolgt. | (automatisch erkannt) |

### Wie die Anzahl dem Kommentar-Thread zugeordnet wird

Der Comment Count-Block verwendet dieselbe Auto-Erkennungslogik wie der **FastComments**-Block:

- Blogartikel-Template: `shopify-article-{article.id}`
- Produkt-Template: `shopify-product-{product.id}`
- Andere Templates: der Anfragepfad

Wenn Sie auf einer Seite eine **Custom URL ID** am **FastComments**-Block gesetzt haben, setzen Sie dieselbe Custom URL ID am Comment Count-Block, damit beide auf denselben Thread zeigen.

### Tipps

- Die Anzahlen für alle Elemente auf der Seite werden in einer einzigen Anfrage abgerufen, sodass das Hinzufügen des Blocks zu jedem Element in einer langen Liste keine zusätzlichen Round-Trip-Kosten verursacht.
- Ein Comment Count-Block pro Artikel oder Produkt in einer Liste ist die erwartete Verwendung; der Block kann so oft hinzugefügt werden, wie Sie benötigen.