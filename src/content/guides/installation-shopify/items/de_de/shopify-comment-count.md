Der **FastComments - Comment Count** Block zeigt eine kleine Kommentarzählung für eine einzelne Seite an. Verwenden Sie ihn in Blog-Beitragslisten, Produktkarten oder jeder Vorlage, die auf eine Seite mit Kommentaren verlinkt, damit Besucher sehen können, wie aktiv jeder Thread ist, bevor sie darauf klicken.

### Add the block

1. Öffnen Sie den Shopify Theme-Editor.
2. Öffnen Sie die Vorlage, in der die Zählung erscheinen soll. Zum Beispiel die **Blog**-Vorlage (die Beitragsliste) oder einen Produktauflistungsabschnitt.
3. Klicken Sie in dem Abschnitt, der jedes Element rendert, auf **Add block**.
4. Unter **Apps** wählen Sie **FastComments - Comment Count**.
5. Klicken Sie auf **Save**.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Überschreibt, aus welchem FastComments-Tenant die Zählung gelesen wird. Leer lassen, um den automatisch konfigurierten Tenant des Shops zu verwenden. | (blank) |
| Custom URL ID | Überschreibt die Seitenkennung, die die Zählung abfragt. Verwenden Sie dies, wenn die Zählung sich auf einer anderen Seite befindet als der FastComments-Block, dem sie folgt. | (auto-detected) |

### How the count matches the comment thread

Der Comment Count block verwendet dieselbe automatische Erkennungslogik wie der **FastComments** Block:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

Wenn Sie eine **Custom URL ID** im **FastComments** Block auf einer Seite setzen, setzen Sie dieselbe Custom URL ID im Comment Count block, damit sie auf denselben Thread zeigen.

### Tips

- Die Zählungen für alle Elemente auf der Seite werden in einer Anfrage abgerufen, sodass das Hinzufügen des Blocks zu jedem Element in einer langen Liste keine zusätzlichen Roundtrips verursacht.
- Ein Comment Count block pro Artikel oder Produkt in einer Liste ist die erwartete Verwendung; der Block kann so oft hinzugefügt werden, wie Sie ihn benötigen.