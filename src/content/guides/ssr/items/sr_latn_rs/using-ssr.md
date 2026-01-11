Da biste koristili FastComments SSR, klijent može da preuzme HTML sa `https://fastcomments.com/ssr/comments` krajnje tačke.

Ovo se može uraditi na više načina.

### Sa WordPress-om

SSR je omogućen po defaultu za korisnike koji imaju isključen JS, kao rezervna opcija u WordPress dodatku od verzije `3.10.2`.

### Na web stranici

Za već postojeću aplikaciju, SSR se može dodati pomoću [sledećeg primera](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), pod pretpostavkom da je korišćen PHP:

[inline-code-attrs-start title = 'Primer SSR-a zasnovan na PHP-u'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Takođe možemo prikazati SSR UI samo kada korisnik ima isključen JS:

[inline-code-attrs-start title = 'Primer rezervne SSR implementacije zasnovane na PHP-u'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Za primer koji koristi SSO, [pogledajte ovde](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Sa prethodno renderovanim sadržajem

Naš blog se generiše tokom izgradnje i pruža [dobar primer SSR-a sa Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Osnovni parametri

Osnovni parametri koje treba proslediti su:
- `tenantId` - Ovo vas identifikuje kao kupca.
- `urlId` - Ovo identifikuje stranicu ili članak za koji se učitavaju komentari i definiše gde se oni čuvaju.
- `url` - Ovo se koristi za obaveštenja i povezane funkcije da vrati link na nit komentara.

### Prilagođavanje stila

SSR verzija widgeta za komentare koristi istu strukturu i engine za renderovanje kao i JavaScript verzija.

Prema tome, sav prilagođeni stil koji radi za JavaScript widget za komentare radi i za SSR. 

### Napomene

Sa SSR-om nema JavaScripta koji kontroliše visinu renderovanog kontejnera. U pregledačima se može pojaviti vertikalni scrollbar za duge diskusije.

Stoga, ovo morate podesiti po potrebi.