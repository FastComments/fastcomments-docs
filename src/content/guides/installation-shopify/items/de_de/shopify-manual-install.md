---
Wenn Sie die [Shopify App Store app](https://apps.shopify.com/fastcomments) nicht installieren können, können Sie FastComments weiterhin hinzufügen, indem Sie Ihr Theme bearbeiten. Dieser Weg ist nützlich, wenn Sie einen FastComments-Tenant, den Sie bereits besitzen, anbinden möchten, oder wenn Sie auf einem Shopify-Storefront einbetten, wo die App keine Option ist.

Die Installation über die App ist der unterstützte Weg für die meisten Shops. Greifen Sie nur darauf zurück, wenn die App nicht passt.

### Schritt 1: Deaktivieren Sie die nativen Kommentare von Shopify

In Ihrem Shopify-Admin, gehen Sie zu **Blog posts > Manage blogs**, öffnen Sie jeden Blog und stellen Sie im rechten Bereich **Comments are disabled** ein. Speichern.

Dies verhindert, dass die eingebaute Kommentarfunktion von Shopify neben FastComments angezeigt wird.

### Schritt 2: Öffnen Sie die Blog-Theme-Vorlage

In Ihrem Shopify-Admin:

1. Gehen Sie zu **Online Store > Themes**.
2. Unter Ihrem aktuellen Theme klicken Sie auf **Actions > Edit code**.
3. Öffnen Sie im Dateibrowser links **Sections** und klicken Sie auf `main-article.liquid`.

Dies ist die Vorlage, die Shopify für einen einzelnen Blogartikel verwendet.

### Schritt 3: Fügen Sie das FastComments-Snippet ein

Scrollen Sie ungefähr zu Zeile 100 von `main-article.liquid`, direkt nach dem schließenden `</div>` des Artikels. Fügen Sie das folgende Snippet ein:

[inline-code-attrs-start title = 'Shopify FastComments-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Ersetzen Sie `"demo"` durch Ihre eigene Tenant ID von [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Klicken Sie auf **Save**.

### Schritt 4: Autorisieren Sie Ihre Shop-Domain

Öffnen Sie einen Blogbeitrag in Ihrem Live-Shop. Wenn Sie anstelle des Kommentarwidgets eine Autorisierungsfehlermeldung sehen, muss FastComments wissen, dass Ihr Shop berechtigt ist, diesen Tenant zu verwenden. Siehe [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors).

### Hinzufügen von FastComments zu anderen Seiten

Dasselbe Snippet funktioniert in jeder Liquid-Vorlage, einschließlich Produktseiten, benutzerdefinierten Seiten und der Startseite. Fügen Sie es dort ein, wo Kommentare erscheinen sollen, und passen Sie `urlId` an, wenn Sie pro Seite einen stabilen Bezeichner möchten (z. B. `urlId: "{{ product.id }}"` in einer Produktvorlage).

---