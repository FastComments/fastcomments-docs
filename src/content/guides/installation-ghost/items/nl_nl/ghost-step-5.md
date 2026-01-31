Vervolgens moeten we bepalen waar de FastComments.com widget-code moet worden toegevoegd.

Als je het standaardthema `casper` gebruikt, zie je een sectie zoals deze op regel `82`:

<div class="screenshot white-bg">
    <div class="title">Uitgeschakelde Reactiesectie</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Uitgeschakelde Reactiesectie" />
</div>

Als je andere thema's gebruikt, zie je dit niet en moet je deze code toevoegen na de laatste `</section>`:

[inline-code-attrs-start title = 'Voorbeeld sectie'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Je zou zoiets klaar moeten hebben:

<div class="screenshot white-bg">
    <div class="title">Sjabloon klaar voor reactiecode</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Sjabloon klaar voor reactiecode" />
</div>

Zodra je klaar bent, kopieer je de FastComments.com widget-code:

[inline-code-attrs-start title = 'Ghost FastComments.com codefragment voor reacties'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...en het zou er als volgt uit moeten zien:

<div class="screenshot white-bg">
    <div class="title">Voeg FastComments.com reactiecode toe</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Voeg FastComments.com reactiecode toe" />
</div>

Klaar met coderen. Nu hoeven we alleen nog ons thema opnieuw te importeren!