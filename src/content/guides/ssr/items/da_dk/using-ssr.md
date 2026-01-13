For at bruge FastComments SSR kan klienten hente HTML fra `https://fastcomments.com/ssr/comments` endpointet.

Dette kan gøres på flere måder.

### Med WordPress

SSR er aktiveret som standard for brugere uden JS som en fallback i WordPress-plugin'et siden version `3.10.2`.

### På en webside

Med en allerede eksisterende applikation kan SSR tilføjes med [følgende eksempel](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), forudsat at sproget er PHP:

[inline-code-attrs-start title = 'PHP-baseret SSR-eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Vi kan også kun vise SSR-brugergrænsefladen, når brugeren har JS deaktiveret:

[inline-code-attrs-start title = 'PHP-baseret SSR-fallback-eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

For et eksempel der bruger SSO, [se her](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Med præ-renderet indhold

Vores blog genereres ved build-tid og giver et [godt eksempel på SSR med Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### De grundlæggende parametre

De grundlæggende parametre, som du skal sende, er:
- `tenantId` - Dette identificerer dig som kunde.
- `urlId` - Dette identificerer siden eller artiklen, der skal indlæses kommentarer til, og bestemmer hvor de gemmes.
- `url` - Dette bruges til notifikationer og relaterede funktioner til at linke tilbage til kommentartråden.

### Tilpasset styling

SSR-versionen af kommentars-widget'en bruger samme struktur og gengivelsesmotor som JavaScript-versionen.

Derfor virker alle tilpassede stilarter, der fungerer for JavaScript-kommentar-widget'en, også for SSR. 

### Bemærkninger

Med SSR er der ingen JavaScript til at kontrollere højden af den gengivne container. I browsere kan en lodret rullebjælke vises ved lange diskussioner.

Derfor skal du justere dette efter behov.