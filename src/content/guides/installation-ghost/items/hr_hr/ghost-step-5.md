---
Sljedeće trebamo utvrditi gdje dodati kod widgeta FastComments.com.

Ako koristite zadanu temu `casper`, vidjet ćete odjeljak poput ovog na liniji `82`:

<div class="screenshot white-bg">
    <div class="title">Onemogućen odjeljak komentara</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Onemogućen odjeljak komentara" />
</div>

Ako koristite druge teme, nećete ovo vidjeti i morat ćete dodati ovaj kod nakon posljednjeg `</section>`:

[inline-code-attrs-start title = 'Primjer odjeljka'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Trebali biste imati nešto ovako spremno:

<div class="screenshot white-bg">
    <div class="title">Predložak spreman za kod komentara</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Predložak spreman za kod komentara" />
</div>

Kada ste spremni, kopirajte kod widgeta FastComments.com:

[inline-code-attrs-start title = 'Isječak koda komentara za Ghost FastComments.com'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...i trebalo bi izgledati ovako:

<div class="screenshot white-bg">
    <div class="title">Dodajte kod komentara FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Dodajte kod komentara FastComments.com" />
</div>

Kodiranje gotovo. Sada samo trebamo ponovno uvesti našu temu!

---