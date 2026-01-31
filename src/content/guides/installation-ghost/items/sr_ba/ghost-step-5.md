Сљедеће морамо утврдити гдје додати код FastComments.com видгета.

Ако користите подразумевану тему `casper`, видећете одјељак попут овог на линији `82`:

<div class="screenshot white-bg">
    <div class="title">Онемогућен одељак коментара</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Онемогућен одељак коментара" />
</div>

Ако користите друге теме, нећете видјети ово, и мораћете да додате овај код након последњег `</section>`:

[inline-code-attrs-start title = 'Примјер одељка'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Требало би да имате нешто овако спремно:

<div class="screenshot white-bg">
    <div class="title">Шаблон спреман за код коментара</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Шаблон спреман за код коментара" />
</div>

Када сте спремни, копирајте код FastComments.com видгета:

[inline-code-attrs-start title = 'Ghost FastComments.com исјечак кода за коментаре'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Додајте FastComments.com код за коментаре</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Додајте FastComments.com код за коментаре" />
</div>

Кодирање завршено. Сада само треба да поново увеземо нашу тему!