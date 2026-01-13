Successivamente dobbiamo identificare dove aggiungere il codice del widget FastComments.com.

Se stai usando il tema predefinito `casper`, vedrai una sezione come questa alla riga `82`:

<div class="screenshot white-bg">
    <div class="title">Sezione commenti disabilitata</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Sezione commenti disabilitata" />
</div>

Se stai usando altri temi, non vedrai questo, e dovrai aggiungere questo codice dopo l'ultimo `</section>`:

[inline-code-attrs-start title = 'Esempio di sezione'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Dovresti avere qualcosa del genere pronto:

<div class="screenshot white-bg">
    <div class="title">Modello pronto per il codice dei commenti</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Modello pronto per il codice dei commenti" />
</div>

Una volta pronto, copia il codice del widget FastComments.com:

[inline-code-attrs-start title = 'Snippet di codice FastComments.com per Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
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

        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        });
    })();
</script>
[inline-code-end]

...e dovrebbe apparire così:

<div class="screenshot white-bg">
    <div class="title">Aggiungi il codice dei commenti FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Aggiungi il codice dei commenti FastComments.com" />
</div>

Codifica completata. Ora dobbiamo solo reimportare il nostro tema!