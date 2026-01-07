Il Widget per il Conteggio di Commenti in Blocco e progettato per visualizzare in modo efficiente i conteggi dei commenti per piu pagine sulla stessa pagina. Invece di effettuare chiamate API individuali per ogni conteggio di commenti, questo widget raggruppa le richieste per prestazioni ottimali.

## Installazione Base

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Come Funziona

Il widget in blocco funziona:

1. Scansionando la pagina per elementi con la classe `fast-comments-count`
2. Leggendo l'attributo `data-fast-comments-url-id` da ogni elemento
3. Raggruppando le richieste API per recuperare piu conteggi di commenti in modo efficiente
4. Aggiornando ogni elemento con il conteggio dei commenti appropriato

## Opzioni di Configurazione

La funzione `FastCommentsCommentCountBulk` accetta le seguenti opzioni di configurazione:

- **tenantId** (obbligatorio): Il tuo ID tenant FastComments
- **apiHost** (opzionale): Host API personalizzato se stai usando un'istanza self-hosted

## Esempio dal Mondo Reale

Ecco un esempio pratico che mostra come potresti usare il widget in blocco in un elenco di post del blog:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Considerazioni sulle Prestazioni

Il widget in blocco ottimizza automaticamente le prestazioni tramite:

- **Raggruppamento delle richieste**: Piu conteggi di commenti vengono recuperati in una singola chiamata API
- **Limiti di dimensione delle richieste**: Le richieste vengono automaticamente suddivise se l'elenco degli URL diventa troppo grande (oltre 1.000 caratteri)
- **Deduplicazione**: Piu elementi con lo stesso `data-fast-comments-url-id` condividono lo stesso conteggio

## Piu Elementi con lo Stesso URL ID

Puoi avere piu elementi sulla pagina con lo stesso `data-fast-comments-url-id`. Saranno tutti aggiornati con lo stesso conteggio:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## Localizzazione

Il widget in blocco formatta automaticamente i conteggi dei commenti in base alle impostazioni della lingua di FastComments. Fornisce testo appropriato per:

- Zero commenti
- Un commento
- Piu commenti

## Quando Usare il Widget in Blocco vs Singolo

**Usa il Widget in Blocco quando:**
- Hai piu conteggi di commenti sulla stessa pagina
- Stai visualizzando un elenco di post/articoli con conteggi di commenti
- Le prestazioni sono importanti (riduce le chiamate API)

**Usa il Widget Singolo quando:**
- Hai bisogno di un solo conteggio di commenti sulla pagina
- Hai bisogno di aggiornamenti in tempo reale (il widget singolo supporta aggiornamenti in tempo reale)
- Vuoi piu controllo sul comportamento del singolo widget
