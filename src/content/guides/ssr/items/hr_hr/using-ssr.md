Za korištenje FastComments SSR-a, klijent može dohvatiti HTML s endpointa `https://fastcomments.com/ssr/comments`.

To se može učiniti na nekoliko načina.

### S WordPressom

SSR je omogućen prema zadanim postavkama kao rezervna opcija (fallback) za korisnike koji nemaju omogućeni JS u WordPress dodatku od verzije `3.10.2`.

### Na web-stranici

U već postojećoj aplikaciji, SSR se može dodati koristeći [sljedeći primjer](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), pod pretpostavkom da je korišteni jezik PHP:

[inline-code-attrs-start title = 'Primjer SSR-a temeljeno na PHP-u'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Također možemo prikazati SSR korisničko sučelje samo kada korisnik ima onemogućen JS:

[inline-code-attrs-start title = 'Primjer fallback rezervne SSR implementacije temeljene na PHP-u'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Za primjer koji koristi SSO, [pogledajte ovdje](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### S unaprijed renderiranim sadržajem

Naš blog se generira u vrijeme buildanja, i pruža [dobar primjer SSR-a s Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Osnovni parametri

Osnovni parametri koje morate proslijediti su:
- `tenantId` - Ovo vas identificira kao klijenta.
- `urlId` - Ovo identificira stranicu ili članak za koji će se učitati komentari i određuje gdje se pohranjuju.
- `url` - Koristi se za obavijesti i srodne značajke kako bi se povezalo natrag na nit komentara.

### Prilagođeno stiliziranje

SSR verzija komentarskog widgeta koristi istu strukturu i mehanizam renderiranja kao JavaScript verzija.

Stoga, svi prilagođeni stilovi koji funkcioniraju za JavaScript komentarski widget funkcioniraju i za SSR. 

### Napomene

S SSR-om nema JavaScripta koji kontrolira visinu renderiranog kontejnera. U preglednicima može se pojaviti vertikalni klizač za duge rasprave.

Stoga, morate po potrebi prilagoditi ovo prema želji.