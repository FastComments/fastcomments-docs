Сада када смо у едитору шаблона, морамо да одлучимо где желимо да прикажемо коментаре или ћаскање уживо.

У овом примеру ћемо га додати непосредно испод видеа. Прелазите курсором преко елемента да бисте додали виџет на његов крај, и кликните `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Додај елемент</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Додај елемент" />
</div>

Изаберите `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Изаберите CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Изаберите CUSTOM JS/HTML" />
</div>

Сада хајде да отворимо едитор кода у који ћемо налепити наш код.

ClickFunnels је у овом кораку помало збуњујућ.

Важно је да *НЕ* изаберете `Code` када прелазите курсором преко новог елемента. Уместо тога, изаберите `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Изаберите SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Изаберите SETTINGS" />
</div>

Сада на десној страни можемо кликнути `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Кликните Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Кликните Open Code Editor" />
</div>

Видећете да се појављује велики квадрат. Овде можемо налепити наш код. Копирајте следећи исечак (користите дугме за копирање у горњем десном углу):

[inline-code-attrs-start title = 'ClickFunnels Streaming Chat кодни исечак'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // неки провајдери мењају исечак кода да би био асинхрон
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

Овај исечак кода је за наш производ Streaming Chat, који добро иде уз видео. Ако желите уместо тога исечак кода за виџет Live Commenting, који најбоље функционише са обичним страницама или блог постовима, он се налази на крају овог туторијала.

Када налепимо исечак кода у прозор, требало би да изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Налепите код</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Налепите код" />
</div>

Сада само треба да затворимо поље:

<div class="screenshot white-bg">
    <div class="title">Затвори</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Затвори" />
</div>

Сада можете прегледати своје измене! Слободно преместите виџет и погледајте где вам највише одговара.

<div class="screenshot white-bg">
    <div class="title">Преглед</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Преглед" />
</div>

Успешно! Не заборавите да тестирате на мобилним уређајима!

<div class="screenshot white-bg">
    <div class="title">Успешно!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Успешно!" />
</div>