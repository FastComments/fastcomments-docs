Vervolgens moeten we bepalen waar we de FastComments.com-widgetcode moeten toevoegen.

Als je het standaardthema `casper` gebruikt, zie je een sectie zoals deze op regel `82`:

<div class="screenshot white-bg">
    <div class="title">Uitgeschakelde reactiesectie</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Uitgeschakelde reactiesectie" />
</div>

Als je andere thema's gebruikt, zie je dit niet en moet je deze code toevoegen na de laatste `</section>`:

[inline-code-attrs-start title = 'Sectievoorbeeld'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Je zou iets zoals dit klaar moeten hebben:

<div class="screenshot white-bg">
    <div class="title">Sjabloon klaar voor commentaarkode</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Sjabloon klaar voor commentaarkode" />
</div>

Als je klaar bent, kopieer dan de FastComments.com-widgetcode:

[inline-code-attrs-start title = 'Ghost FastComments.com Reactiecodefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...en het zou er zo uit moeten zien:

<div class="screenshot white-bg">
    <div class="title">Voeg FastComments.com-reactiecode toe</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Voeg FastComments.com-reactiecode toe" />
</div>

Coderen voltooid. Nu hoeven we alleen nog ons thema opnieuw te importeren!