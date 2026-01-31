Далі потрібно визначити, куди додати код віджета FastComments.com.

Якщо ви використовуєте стандартну тему `casper`, ви побачите секцію, схожу на цю, на рядку `82`:

<div class="screenshot white-bg">
    <div class="title">Відключений розділ коментарів</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Відключений розділ коментарів" />
</div>

Якщо ви використовуєте інші теми, ви не побачите цього й вам потрібно додати цей код після останнього `</section>`:

[inline-code-attrs-start title = 'Приклад секції'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

У вас має бути щось на кшталт цього:

<div class="screenshot white-bg">
    <div class="title">Шаблон готовий для коду коментарів</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Шаблон готовий для коду коментарів" />
</div>

Коли все готово, скопіюйте код віджета FastComments.com:

[inline-code-attrs-start title = 'Фрагмент коду коментарів FastComments.com для Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...і це має виглядати приблизно так:

<div class="screenshot white-bg">
    <div class="title">Додати код коментарів FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Додати код коментарів FastComments.com" />
</div>

Кодування завершено. Тепер нам залишилося лише знову імпортувати нашу тему!

---