Successivamente dobbiamo identificare dove aggiungere il codice del widget FastComments.com.

Se stai usando il tema predefinito `casper`, vedrai una sezione come questa alla riga `82`:

<div class="screenshot white-bg">
    <div class="title">Sezione Commenti Disabilitata</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Sezione Commenti Disabilitata" />
</div>

Se usi altri temi, non vedrai questo e dovrai aggiungere questo codice dopo l'ultimo `</section>`:

[inline-code-attrs-start title = 'Esempio di sezione'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Dovresti avere qualcosa di simile pronto:

<div class="screenshot white-bg">
    <div class="title">Template Pronto per il Codice dei Commenti</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Template Pronto per il Codice dei Commenti" />
</div>

Una volta pronto, copia il codice del widget FastComments.com:

[inline-code-attrs-start title = 'Snippet del codice commenti FastComments.com per Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let simpleSSO = null;

        \{{#if access}}
            \{{#if @member}}
                simpleSSO = {
                    id: '\{{ @member.uuid }}',
                    email: '\{{@member.email}}',
                    username: '\{{@member.name}}',
                    avatar: '\{{ @member.avatar_image }}',
                    optedInNotifications: true,
                    optedInSubscriptionNotifications: true,
                    displayLabel: '\{{@member.labels}}'
                }
            \{{/if}}
        \{{/if}}

        window.fcConfigs = [{
            target: '#fastcomments-widget',
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        }];
    })();
</script>
[inline-code-end]

...e dovrebbe apparire così:

<div class="screenshot white-bg">
    <div class="title">Aggiungi il codice commenti FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Aggiungi il codice commenti FastComments.com" />
</div>

Codice inserito. Ora dobbiamo solo reimportare il nostro tema!

---