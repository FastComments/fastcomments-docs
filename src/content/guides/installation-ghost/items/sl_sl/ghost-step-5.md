Naslednje moramo ugotoviti, kam dodati kodo pripomočka FastComments.com.

Če uporabljate privzeto temo `casper`, boste na vrstici `82` videli razdelek, kot je ta:

<div class="screenshot white-bg">
    <div class="title">Onemogočen razdelek komentarjev</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Onemogočen razdelek komentarjev" />
</div>

Če uporabljate druge teme, tega ne boste videli in boste morali to kodo dodati za zadnjim `</section>`:

[inline-code-attrs-start title = 'Primer razdelka'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Mali pripomoček bi moral biti pripravljen tako:

<div class="screenshot white-bg">
    <div class="title">Predloga pripravljena za kodo komentarjev</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Predloga pripravljena za kodo komentarjev" />
</div>

Ko je pripravljeno, kopirajte kodo pripomočka FastComments.com:

[inline-code-attrs-start title = 'Vstavek kode komentarja FastComments.com za Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...in izgledalo bi takole:

<div class="screenshot white-bg">
    <div class="title">Dodaj kodo komentarjev FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Dodaj kodo komentarjev FastComments.com" />
</div>

Koda je pripravljena. Zdaj moramo le ponovno uvoziti temo!