Nato moramo ugotoviti, kam dodati kodo vtičnika FastComments.com.

Če uporabljate privzeto temo `casper`, boste videli odsek, podoben temu na vrstici `82`:

<div class="screenshot white-bg">
    <div class="title">Onemogočen razdelek komentarjev</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Onemogočen razdelek komentarjev" />
</div>

Če uporabljate druge teme, tega ne boste videli in boste morali to kodo dodati za zadnjo oznako `</section>`:

[inline-code-attrs-start title = 'Primer razdelka'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Morali bi imeti nekaj takega pripravljeno:

<div class="screenshot white-bg">
    <div class="title">Predloga pripravljena za kodo komentarjev</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Predloga pripravljena za kodo komentarjev" />
</div>

Ko je pripravljeno, kopirajte kodo vtičnika FastComments.com:

[inline-code-attrs-start title = 'Izsek kode komentarjev FastComments.com za Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...in naj bi izgledalo takole:

<div class="screenshot white-bg">
    <div class="title">Dodajte kodo komentarjev FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Dodajte kodo komentarjev FastComments.com" />
</div>

Koda je končana. Zdaj moramo le ponovno uvoziti našo temo!