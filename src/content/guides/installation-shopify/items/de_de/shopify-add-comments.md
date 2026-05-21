Der **FastComments**-Block ist das Haupt-Kommentar-Widget. Fügen Sie ihn zu Blog-Post-Vorlagen, Produktvorlagen oder jeder anderen Seite hinzu, auf der Sie einen Diskussions-Thread oder einen Live-Chat möchten.

### Block hinzufügen

1. Öffnen Sie den Shopify Theme-Editor (**Online Store > Themes > Customize**).
2. Wählen Sie die Vorlage, auf der Sie Kommentare haben möchten: **Blog post**, **Product**, oder jede andere Seiten- oder Abschnittsvorlage.
3. Klicken Sie in dem Abschnitt, in dem die Kommentare erscheinen sollen, auf **Add block**.
4. Wählen Sie unter **Apps** **FastComments** aus.
5. Klicken Sie auf **Save**.

Der Block erscheint sofort. Es gibt keine Tenant ID zum Eingeben; der Tenant Ihres Shops wird beim Installieren der App automatisch verbunden.

### Einstellungen

| Setting | Was es bewirkt | Standard |
|---|---|---|
| Tenant ID (optional) | Überschreibt, gegen welchen FastComments Tenant der Block gerendert wird. Leer lassen, um den automatisch konfigurierten Tenant des Shops zu verwenden. Eine manuelle Tenant ID finden Sie unter fastcomments.com/auth/my-account/api-secret. | (blank) |
| SSO | Loggt den Besucher automatisch als dessen Shopify-Kundenkonto ein, bevor er kommentiert. Siehe [Automatisches Login für Shopify-Kunden](/guide-installation-shopify.html#shopify-sso). | On |
| Commenting Style | **Threaded** für verschachtelte Antworten und Abstimmungen, oder **Streaming** für einen Echtzeit-Chat-Feed. | Threaded |
| Custom URL ID | Überschreibt die automatisch erkannte Seitenkennung. Verwenden Sie dies, wenn zwei URLs denselben Kommentarthread teilen sollen. | (auto-detected) |

### Wie der Seitenbezeichner gewählt wird

Jeder Kommentarthread ist durch eine URL-ID gekennzeichnet. Der Block wählt diese automatisch:

- **Blog post template:** `shopify-article-{article.id}`, die stabil bei Änderungen von Slug und Titel ist.
- **Product template:** `shopify-product-{product.id}`, die stabil bei Änderungen von Slug und Titel ist.
- **Other templates:** der Request-Pfad.

Wenn Sie **Custom URL ID** festlegen, wird stattdessen dieser Wert verwendet. Verwenden Sie dieselbe Custom URL ID über mehrere Blöcke hinweg (zum Beispiel bei einer lokalisierten Variante einer Produktseite), um einen Kommentarthread zu teilen.

### Threaded vs Streaming

**Threaded** ist die Standardeinstellung. Besucher antworten einander, können abstimmen, und die Moderationstools funktionieren wie erwartet. Am besten für Blogbeiträge und Produktbewertungen.

**Streaming** verzichtet auf die Thread-Struktur und zeigt neue Kommentare in Echtzeit, sobald sie gepostet werden, ähnlich einem Chat-Feed. Am besten für Produkteinführungen, Live-Events und Community-Seiten.

### Mehrere Blöcke auf derselben Seite

Der Block kann mehrmals derselben Vorlage hinzugefügt werden. Zum Beispiel eine Reviews-Zusammenfassung oben auf einer Produktseite und ein FastComments-Block unten. Die Blöcke teilen eine URL-ID, sodass die Zusammenfassung die Kommentare darunter widerspiegelt.

### Tipps

- Der Block versteckt sich in der Theme-Editor-Vorschau mit einem gelben Hinweis, wenn er keinen Tenant findet. Wenn das in Ihrem Live-Shop erscheint, installieren Sie die FastComments-App neu.
- Auf einer Produktseite fungiert der FastComments-Block gleichzeitig als Ihr Produktbewertungs-Widget. Kombinieren Sie ihn mit **FastComments - Reviews Summary** für eine Sterne-Bewertungszusammenfassung oben auf der Seite.