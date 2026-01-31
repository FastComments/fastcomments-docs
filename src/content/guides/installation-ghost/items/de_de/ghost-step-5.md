Als Nächstes müssen wir ermitteln, wo der FastComments.com-Widget-Code hinzugefügt werden soll.

Wenn Sie das Standard-Theme `casper` verwenden, sehen Sie einen Abschnitt wie diesen bei Zeile `82`:

<div class="screenshot white-bg">
    <div class="title">Deaktivierter Kommentarbereich</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Deaktivierter Kommentarbereich" />
</div>

Wenn Sie andere Themes verwenden, sehen Sie dies nicht und müssen diesen Code nach dem letzten `</section>` hinzufügen:

[inline-code-attrs-start title = 'Beispiel für Abschnitt'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Sie sollten etwa Folgendes bereit haben:

<div class="screenshot white-bg">
    <div class="title">Vorlage bereit für Kommentar-Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Vorlage bereit für Kommentar-Code" />
</div>

Sobald alles bereit ist, kopieren Sie den FastComments.com-Widget-Code:

[inline-code-attrs-start title = 'Ghost FastComments.com Kommentar-Code-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...und es sollte so aussehen:

<div class="screenshot white-bg">
    <div class="title">FastComments.com Kommentar-Code hinzufügen</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="FastComments.com Kommentar-Code hinzufügen" />
</div>

Code fertig. Nun müssen wir nur noch unser Theme erneut importieren!