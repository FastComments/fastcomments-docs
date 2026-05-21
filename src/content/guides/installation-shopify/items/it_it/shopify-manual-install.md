Se non puoi installare l'[app dello Shopify App Store](https://apps.shopify.com/fastcomments), puoi comunque aggiungere FastComments modificando il tuo tema. Questo percorso è utile quando vuoi collegare un tenant FastComments che già possiedi, oppure quando stai incorporando su una vetrina Shopify dove l'app non è un'opzione.

L'installazione tramite app è il percorso supportato per la maggior parte dei negozi. Usalo solo se l'app non è adatta.

### Step 1: Disable Shopify's native comments

Nel pannello di amministrazione di Shopify, vai a **Blog posts > Manage blogs**, apri ogni blog e imposta **Comments are disabled** nel pannello a destra. Salva.

Questo impedisce che il sistema di commenti integrato di Shopify venga mostrato insieme a FastComments.

### Step 2: Open the blog theme template

Nel pannello di amministrazione di Shopify:

1. Vai a **Online Store > Themes**.
2. Sotto il tema corrente, clicca **Actions > Edit code**.
3. Nel file browser a sinistra, apri **Sections** e clicca `main-article.liquid`.

Questo è il template che Shopify renderizza per un singolo articolo del blog.

### Step 3: Paste the FastComments snippet

Scorri fino a circa la riga 100 di `main-article.liquid`, subito dopo la chiusura `</div>` del corpo dell'articolo. Incolla il seguente snippet:

[inline-code-attrs-start title = 'Snippet di FastComments per Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Step 4: Authorize your shop domain

Apri un post del blog sul tuo negozio live. Se vedi un errore di autorizzazione invece del widget dei commenti, FastComments ha bisogno di sapere che il tuo negozio è autorizzato a usare questo tenant. Vedi [Errori di dominio](/guide-installation-shopify.html#shopify-domain-errors).

### Adding FastComments to other pages

Lo stesso snippet funziona su qualsiasi template Liquid, incluse le pagine prodotto, le pagine personalizzate e la homepage. Incollalo dove vuoi che compaiano i commenti e regola `urlId` se desideri un identificatore stabile per pagina (ad esempio, `urlId: "{{ product.id }}"` in un template prodotto).