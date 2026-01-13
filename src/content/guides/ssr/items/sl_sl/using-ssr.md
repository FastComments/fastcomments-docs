Za uporabo FastComments SSR lahko odjemalec pridobi HTML s končne točke `https://fastcomments.com/ssr/comments`.

To je mogoče narediti na več načinov.

### Z WordPressom

SSR je vtičniku za WordPress privzeto omogočen kot rezervna možnost za uporabnike brez vključenega JS od različice `3.10.2`.

### Na spletni strani

V že obstoječo aplikacijo lahko SSR dodate z [naslednjim primerom](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), pod pogojem, da je uporabljen jezik PHP:

[inline-code-attrs-start title = 'Primer SSR v PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Prav tako lahko prikazujemo SSR vmesnik le, ko ima uporabnik onemogočen JS:

[inline-code-attrs-start title = 'Primer rezervne možnosti SSR v PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Za primer z uporabo SSO poglejte [tukaj](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Z vnaprej upodobljeno vsebino

Naš blog se ustvari med izgradnjo in ponuja [dober primer SSR z Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Osnovni parametri

Osnovni parametri, ki jih morate posredovati, so:
- `tenantId` - To vas identificira kot stranko.
- `urlId` - To identificira stran ali članek, za katerega se naložijo komentarji, in določa, kam se shranijo.
- `url` - To se uporablja za obvestila in sorodne funkcije za povezavo nazaj do niti komentarjev.

### Prilagojeno oblikovanje

SSR različica pripomočka za komentarje uporablja enako strukturo in mehanizem za upodabljanje kot JavaScript različica.

Zaradi tega vse prilagojeno oblikovanje, ki deluje za JavaScript pripomoček za komentarje, deluje tudi za SSR. 

### Opombe

Pri SSR ni JavaScripta, ki bi nadziral višino upodobljenega kontejnerja. V brskalnikih se lahko pri dolgih razprav pojavi navpični drsni trak.

Zato morate to po potrebi prilagoditi.