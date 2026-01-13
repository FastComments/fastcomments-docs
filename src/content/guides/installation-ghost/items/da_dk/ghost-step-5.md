Nu skal vi identificere, hvor vi skal tilføje FastComments.com-widgetkoden.

Hvis du bruger standardtemaet `casper`, vil du se en sektion som denne på linje `82`:

<div class="screenshot white-bg">
    <div class="title">Deaktiveret kommentarsektion</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Deaktiveret kommentarsektion" />
</div>

Hvis du bruger andre temaer, vil du ikke se dette, og du bliver nødt til at tilføje denne kode efter den sidste `</section>`:

[inline-code-attrs-start title = 'Eksempel på sektion'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Du bør have noget, der ligner dette, klar:

<div class="screenshot white-bg">
    <div class="title">Skabelon klar til kommentarkode</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Skabelon klar til kommentarkode" />
</div>

Når du er klar, kopier FastComments.com-widgetkoden:

[inline-code-attrs-start title = 'Ghost FastComments.com Kommentar-kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...og det skal se sådan ud:

<div class="screenshot white-bg">
    <div class="title">Tilføj FastComments.com kommentarkode</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Tilføj FastComments.com kommentarkode" />
</div>

Kodningen er færdig. Nu skal vi bare genimportere vores tema!