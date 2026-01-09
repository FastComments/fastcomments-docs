Далее нужно определить, куда добавить код виджета FastComments.com.

Если вы используете тему по умолчанию `casper`, вы увидите секцию, похожую на эту, на строке `82`:

<div class="screenshot white-bg">
    <div class="title">Отключенный раздел комментариев</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Отключенный раздел комментариев" />
</div>

Если вы используете другую тему, вы не увидите этого и потребуется добавить этот код после последнего `</section>`:

[inline-code-attrs-start title = 'Пример секции'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

У вас должно быть что-то вроде этого:

<div class="screenshot white-bg">
    <div class="title">Шаблон готов для кода комментариев</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Шаблон готов для вставки кода комментариев" />
</div>

Когда будете готовы, скопируйте код виджета FastComments.com:

[inline-code-attrs-start title = 'Фрагмент кода комментариев FastComments.com для Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...и это должно выглядеть так:

<div class="screenshot white-bg">
    <div class="title">Добавление кода комментариев FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Добавление кода комментариев FastComments.com" />
</div>

Код готов. Теперь осталось только повторно импортировать тему!