Sledeće treba da utvrdimo gde da dodamo FastComments.com widget kod.

Ako koristite podrazumevanu temu `casper`, videćete odeljak kao ovaj na liniji `82`:

<div class="screenshot white-bg">
    <div class="title">Onemogućen odeljak komentara</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Onemogućen odeljak komentara" />
</div>

Ako koristite druge teme, ovo nećete videti i moraćete da dodate ovaj kod posle poslednjeg `</section>`:

[inline-code-attrs-start title = 'Primer sekcije'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Treba da imate nešto ovako spremno:

<div class="screenshot white-bg">
    <div class="title">Šablon spreman za kod komentara</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Šablon spreman za kod komentara" />
</div>

Kada ste spremni, kopirajte FastComments.com widget kod:

[inline-code-attrs-start title = 'Ghost FastComments.com deo koda za komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...i trebalo bi da izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments.com kod za komentare</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Dodajte FastComments.com kod za komentare" />
</div>

Kodiranje gotovo. Sada samo treba da ponovo uvezemo našu temu!