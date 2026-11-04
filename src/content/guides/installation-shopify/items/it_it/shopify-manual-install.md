Se non riesci a installare l'[app dal Shopify App Store](https://apps.shopify.com/fastcomments), puoi comunque aggiungere FastComments modificando il tuo tema. Questo percorso è utile quando vuoi collegare un tenant FastComments che possiedi già, o quando stai integrando su uno storefront Shopify dove l'app non è un'opzione.

L'installazione basata sull'app è il percorso supportato per la maggior parte dei negozi. Ricorri a questo metodo solo se l'app non è adatta.

### Passo 1: Disabilita i commenti nativi di Shopify

Nell'amministrazione di Shopify, vai su **Blog posts > Manage blogs**, apri ogni blog e imposta **Comments are disabled** nel pannello a destra. Salva.

Questo impedisce che il sistema di commenti integrato di Shopify venga visualizzato insieme a FastComments.

### Passo 2: Apri il template del tema del blog

Nell'amministrazione di Shopify:

1. Vai su **Online Store > Themes**.
2. Sotto il tuo tema attuale, clicca **Actions > Edit code**.
3. Nel file browser a sinistra, apri **Sections** e clicca `main-article.liquid`.

Questo è il template che Shopify renderizza per un singolo articolo del blog.

### Passo 3: Incolla lo snippet di FastComments

Scorri fino a circa la riga 100 di `main-article.liquid`, subito dopo la chiusura `</div>` del corpo dell'articolo. Incolla il seguente snippet:

[inline-code-attrs-start title = 'Snippet Shopify FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sostituisci `"demo"` con il tuo Tenant ID da [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). Clicca **Save**.

### Passo 4: Autorizza il dominio del tuo negozio

Apri un articolo del blog sul tuo negozio live. Se invece del widget dei commenti visualizzi un errore di autorizzazione, FastComments deve sapere che il tuo negozio è autorizzato a usare questo tenant. Vedi [Errori di dominio](/guide-installation-shopify.html#shopify-domain-errors).

### Aggiungere FastComments ad altre pagine

Lo stesso snippet funziona su qualsiasi template Liquid, incluse le pagine prodotto, le pagine personalizzate e la home page. Incollalo dove vuoi che appaiano i commenti e modifica `urlId` se desideri un identificatore stabile per pagina (ad esempio, `urlId: "{{ product.id }}"` su un template di prodotto).