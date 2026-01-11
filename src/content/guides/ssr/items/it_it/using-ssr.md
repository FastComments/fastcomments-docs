Per usare FastComments SSR, il client può ottenere HTML dall'endpoint `https://fastcomments.com/ssr/comments`.

Questo può essere fatto in diversi modi.

### Con WordPress

SSR è abilitato di default per gli utenti senza JS abilitato come fallback nel plugin WordPress dalla versione `3.10.2`.

### In una pagina web

Con un'applicazione già esistente, SSR può essere aggiunto con il [seguente esempio](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr), assumendo che il linguaggio usato sia PHP:

[inline-code-attrs-start title = 'Esempio SSR basato su PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Possiamo anche mostrare l'interfaccia SSR solo quando l'utente ha JS disabilitato:

[inline-code-attrs-start title = 'Esempio di fallback SSR basato su PHP'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

Per un esempio che utilizza SSO, [vedi qui](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Con contenuti pre-renderizzati

Il nostro blog è generato in fase di build e fornisce un [buon esempio di SSR con Handlebars](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51).

### Parametri di base

I parametri di base che devi fornire sono:
- `tenantId` - Questo ti identifica come cliente.
- `urlId` - Questo identifica la pagina o l'articolo per caricare i commenti e definisce dove vengono salvati.
- `url` - Questo è usato per le notifiche e funzionalità correlate per collegarsi al thread dei commenti.

### Stile personalizzato

La versione SSR del widget dei commenti utilizza la stessa struttura e lo stesso motore di rendering di quella JavaScript.

Di conseguenza, tutti gli stili personalizzati che funzionano per il widget di commenti JavaScript funzionano anche per SSR. 

### Note

Con SSR non c'è JavaScript per controllare l'altezza del contenitore renderizzato. Nei browser, potrebbe apparire una barra di scorrimento verticale per discussioni lunghe.

Pertanto, è necessario regolare questo secondo le tue esigenze.

---