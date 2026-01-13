---
Per far funzionare correttamente l'integrazione tra Weebly e FastComments, dobbiamo aggiungere **due** piccoli frammenti di codice.

Il primo frammento serve a nascondere il messaggio di Weebly "I commenti sono chiusi", e il secondo serve a caricare effettivamente FastComments.

Per prima cosa, copia questo piccolo frammento di codice:

[inline-code-attrs-start title = 'Frammento di codice header di FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Poi, nella stessa pagina delle impostazioni vista in `Step One`, clicca il `+` accanto a `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Apri il codice header del post</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Apri il codice header del post" />
</div>

Vedrai aprirsi una casella di testo simile a questa:

<div class="screenshot white-bg">
    <div class="title">Codice header del post aperto</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Codice header del post aperto" />
</div>

Ora incolliamo il nostro frammento di codice:

<div class="screenshot white-bg">
    <div class="title">Frammento di codice header incollato</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Frammento di codice header incollato" />
</div>

Il passo successivo è il codice footer per abilitare FastComments. Clicca il segno più accanto a `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Apri il codice footer del post</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Apri il codice footer del post" />
</div>

Copia questo frammento di codice progettato **specificamente per Weebly**:

[inline-code-attrs-start title = 'Frammento di codice footer di FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // rimuovi il pulsante mostra commenti
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Ora incolliamo il nostro codice footer:

<div class="screenshot white-bg">
    <div class="title">Codice footer del post aggiunto</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Codice footer del post aggiunto" />
</div>

Questo è tutto!

---