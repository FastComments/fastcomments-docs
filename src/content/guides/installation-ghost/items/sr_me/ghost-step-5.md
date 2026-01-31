Следеће треба да утврдимо где да додамо FastComments.com видгет код.

Ако користите подразумевану тему `casper`, видећете одељак сличан овом на линији `82`:

<div class="screenshot white-bg">
    <div class="title">Онемогућен одељак за коментаре</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Онемогућен одељак за коментаре" />
</div>

Ако користите друге теме, нећете то видети и мораћете да додате овај код након последњег `</section>`:

[inline-code-attrs-start title = 'Пример секције'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Требало би да имате нешто овако спремно:

<div class="screenshot white-bg">
    <div class="title">Шаблон спреман за код коментара</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Шаблон спреман за код коментара" />
</div>

Када сте спремни, копирајте FastComments.com видгет код:

[inline-code-attrs-start title = 'Ghost FastComments.com исечак кода за коментаре'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...and it should look like this:

<div class="screenshot white-bg">
    <div class="title">Додајте FastComments.com код за коментаре</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Додајте FastComments.com код за коментаре" />
</div>

Кодирање је завршено. Сада само треба поново да увеземо нашу тему!