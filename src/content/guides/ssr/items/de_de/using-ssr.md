---
Um FastComments SSR zu verwenden, kann der Client HTML vom Endpunkt `https://fastcomments.com/ssr/comments` abrufen.

Dies kann auf verschiedene Weise erfolgen.

### Mit WordPress

SSR ist standardmäßig als Fallback im WordPress-Plugin für Benutzer bei deaktiviertem JS seit Version `3.10.2` aktiviert.

### In einer Webseite

Bei einer bereits bestehenden Anwendung kann SSR mit dem [folgenden Beispiel](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr) hinzugefügt werden, vorausgesetzt, die verwendete Sprache ist PHP:

[inline-code-attrs-start title = 'PHP-basiertes SSR-Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

Wir können die SSR-Benutzeroberfläche auch nur anzeigen, wenn der Benutzer JS deaktiviert hat:

[inline-code-attrs-start title = 'PHP-basiertes SSR-Fallback-Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

Für ein Beispiel mit SSO, [siehe hier](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Mit vorgerendertem Inhalt

Unser Blog wird zur Build-Zeit generiert und bietet ein [gutes Beispiel für SSR mit Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Die grundlegenden Parameter

Die grundlegenden Parameter, die Sie übergeben müssen, sind:
- `tenantId` - Dies identifiziert Sie als Kunden.
- `urlId` - Dies identifiziert die Seite oder den Artikel, für den Kommentare geladen werden sollen, und legt fest, wo sie gespeichert werden.
- `url` - Dies wird für Benachrichtigungen und verwandte Funktionen verwendet, um auf den Kommentar-Thread zurückzuverlinken.

### Benutzerdefiniertes Styling

Die SSR-Version des Kommentar-Widgets verwendet die gleiche Struktur und Rendering-Engine wie die JavaScript-Version.

Dementsprechend funktioniert sämtliches benutzerdefiniertes Styling, das für das JavaScript-Kommentar-Widget funktioniert, auch für SSR. 

### Hinweise

Bei SSR gibt es kein JavaScript, das die Höhe des gerenderten Containers steuert. In Browsern kann bei langen Diskussionen eine vertikale Bildlaufleiste angezeigt werden.

Passen Sie dies entsprechend an.

---