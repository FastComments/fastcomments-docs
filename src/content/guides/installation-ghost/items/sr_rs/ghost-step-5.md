Следеће треба да идентификујемо где да додамо код видгета са FastComments.com.

Ако користите подразумевану `casper` тему, видећете одељак сличан овом на линији `82`:

<div class="screenshot white-bg">
    <div class="title">Одељак за коментаре онемогућен</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Одељак за коментаре онемогућен" />
</div>

Ако користите друге теме, нећете видети ово, и потребно је да додате овај код после последњег `</section>`:

[inline-code-attrs-start title = 'Пример секције'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Требало би да имате спремно нешто овако:

<div class="screenshot white-bg">
    <div class="title">Шаблон спреман за код коментара</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Шаблон спреман за код коментара" />
</div>

Када је спремно, копирајте код видгета са FastComments.com:

[inline-code-attrs-start title = 'Фрагмент кода коментара FastComments.com за Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...и требало би да изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Додавање кода за коментаре FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Додавање кода за коментаре FastComments.com" />
</div>

Кодирање завршено. Сада само треба да поново увеземо нашу тему!