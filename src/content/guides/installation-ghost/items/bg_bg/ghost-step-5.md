След това трябва да определим къде да добавим кода на уиджета FastComments.com.

Ако използвате стандартната тема `casper`, ще видите секция като тази на ред `82`:

<div class="screenshot white-bg">
    <div class="title">Деактивирана секция за коментари</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Деактивирана секция за коментари" />
</div>

Ако използвате друга тема, няма да видите това и ще трябва да добавите този код след последния `</section>`:

[inline-code-attrs-start title = 'Пример за секция'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Трябва да имате нещо подобно готово:

<div class="screenshot white-bg">
    <div class="title">Шаблон готов за код на коментари</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Шаблон готов за код на коментари" />
</div>

Когато сте готови, копирайте кода на уиджета FastComments.com:

[inline-code-attrs-start title = 'Фрагмент код за коментари на FastComments.com за Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...и трябва да изглежда така:

<div class="screenshot white-bg">
    <div class="title">Добавяне на код за коментари на FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Добавяне на код за коментари на FastComments.com" />
</div>

Кодирането е готово. Сега просто трябва да реимпортираме темата!