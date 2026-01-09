---
Następnie musimy określić, gdzie dodać kod widżetu FastComments.com.

Jeśli używasz domyślnego motywu `casper`, zobaczysz sekcję podobną do tej w linii `82`:

<div class="screenshot white-bg">
    <div class="title">Wyłączona sekcja komentarzy</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Wyłączona sekcja komentarzy" />
</div>

Jeśli używasz innego motywu, nie zobaczysz tego i będziesz musiał dodać ten kod po ostatnim `</section>`:

[inline-code-attrs-start title = 'Przykład sekcji'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Powinieneś mieć coś takiego gotowe:

<div class="screenshot white-bg">
    <div class="title">Szablon gotowy na kod komentarzy</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Szablon gotowy na kod komentarzy" />
</div>

Gdy będziesz gotowy, skopiuj kod widżetu FastComments.com:

[inline-code-attrs-start title = 'Fragment kodu komentarzy FastComments.com dla Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...i powinno to wyglądać tak:

<div class="screenshot white-bg">
    <div class="title">Dodaj kod komentarzy FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Dodaj kod komentarzy FastComments.com" />
</div>

Kodowanie zakończone. Teraz trzeba tylko ponownie zaimportować nasz motyw!

---