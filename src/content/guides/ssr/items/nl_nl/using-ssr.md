Om FastComments SSR te gebruiken, kan de client HTML ophalen van het endpoint `https://fastcomments.com/ssr/comments`.

Dit kan op verschillende manieren.

### Met WordPress

SSR is standaard ingeschakeld als fallback voor gebruikers zonder ingeschakelde JS in de WordPress-plugin sinds versie `3.10.2`.

### In een webpagina

Bij een bestaande toepassing kan SSR worden toegevoegd met het [volgende voorbeeld](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), ervan uitgaande dat de gebruikte taal PHP is:

[inline-code-attrs-start title = 'PHP-gebaseerd SSR-voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

We kunnen de SSR-UI ook alleen tonen wanneer de gebruiker JavaScript heeft uitgeschakeld:

[inline-code-attrs-start title = 'PHP-gebaseerd SSR-terugvalvoorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Voor een voorbeeld met SSO, [zie hier](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Met vooraf-gerenderde inhoud

Onze blog wordt tijdens het bouwen gegenereerd en biedt een [goed voorbeeld van SSR met Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### De basisparameters

De basisparameters die u moet doorgeven zijn:
- `tenantId` - Dit identificeert u als klant.
- `urlId` - Dit identificeert de pagina of het artikel waarvoor reacties moeten worden geladen, en bepaalt waar ze worden opgeslagen.
- `url` - Dit wordt gebruikt voor meldingen en gerelateerde functies om terug te linken naar de reactiedraad.

### Aangepaste styling

De SSR-versie van de reactiewidget gebruikt dezelfde structuur en rendering-engine als de JavaScript-versie.

Als zodanig werkt alle aangepaste styling die voor de JavaScript-reactiewidget werkt ook voor SSR. 

### Opmerkingen

Bij SSR is er geen JavaScript om de hoogte van de gerenderde container te regelen. In browsers kan bij lange discussies een verticale schuifbalk verschijnen.

Pas dit dus naar wens aan.

---