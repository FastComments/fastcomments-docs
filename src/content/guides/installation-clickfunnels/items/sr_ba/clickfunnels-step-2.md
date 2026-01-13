Сада када смо у уређивачу шаблона, морамо одлучити гдје желимо приказати коментаре или уживо ћаскање.

У овом примјеру ћемо га додати директно испод видеа. Помјерите курсор преко елемента на који ћете додати видгет (на крај), и кликните `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Додај елемент</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Додај елемент" />
</div>

Изаберите `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Изаберите CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Изаберите CUSTOM JS/HTML" />
</div>

Сада хајде да отворимо уређивач кода у који ћемо залепити наш код.

ClickFunnels може бити мало збуњујући у овом наредном кораку.

Важно је да *НЕ* изаберете `Code` када помјерите курсор преко новог елемента. Умјесто тога, изаберите `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Изаберите SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Изаберите SETTINGS" />
</div>

Сада на десној страни можемо кликнути `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Кликните Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Кликните Open Code Editor" />
</div>

Видећете да се отвори велики квадрат. Овдје можемо залепити наш код. Копирајте следећи исјечак (користите дугме за копирање у горњем десном углу):

[inline-code-attrs-start title = 'Примјер кода за ClickFunnels Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // неки провајдери мијењају исјечак кода да буде асинхроно
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Овај исјечак кода је за наш Streaming Chat производ, који се добро слаже са видеима. Ако желите умјесто тога исјечак кода за видгет Live Commenting, који најбоље пристаје са обичним страницама или блог постовима, он се налази на крају овог туторијала.

Када залепимо исјечак кода у прозор, требало би да изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Залепите код</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Залепите код" />
</div>

Сада само треба да затворимо прозор:

<div class="screenshot white-bg">
    <div class="title">Затвори</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Затвори" />
</div>

Сада можете прегледати своје промјене! Слободно помјерајте видгет и видите гдје вам највише одговара.

<div class="screenshot white-bg">
    <div class="title">Преглед</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Преглед" />
</div>

Успјех! Не заборавите тестирати на мобилним уређајима!

<div class="screenshot white-bg">
    <div class="title">Успјех!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Успјех!" />
</div>