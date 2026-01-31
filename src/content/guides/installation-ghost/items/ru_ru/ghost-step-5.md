Далее необходимо определить, куда добавить код виджета FastComments.com.

Если вы используете тему по умолчанию `casper`, вы увидите раздел, похожий на этот, на строке `82`:

<div class="screenshot white-bg">
    <div class="title">Отключённый раздел комментариев</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Отключённый раздел комментариев" />
</div>

Если вы используете другую тему, вы не увидите это и вам потребуется добавить этот код после последнего тега `</section>`:

[inline-code-attrs-start title = 'Пример секции'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

У вас должно получиться примерно следующее:

<div class="screenshot white-bg">
    <div class="title">Шаблон готов к вставке кода комментариев</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Шаблон готов к вставке кода комментариев" />
</div>

Когда всё будет готово, скопируйте код виджета FastComments.com:

[inline-code-attrs-start title = 'Фрагмент кода комментариев FastComments.com для Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...и это должно выглядеть так:

<div class="screenshot white-bg">
    <div class="title">Добавление кода комментариев FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Добавление кода комментариев FastComments.com" />
</div>

Кодирование завершено. Теперь осталось только повторно импортировать нашу тему!