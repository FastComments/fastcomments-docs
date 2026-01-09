Сега, когато сме в редактора на шаблони, трябва да решим къде искаме да показваме коментарите или чата на живо.

В този пример ще го добавим директно под видеото. Задръжте курсора върху елемента, за да добавите уиджета в края му, и кликнете `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Добавяне на елемент</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Добавяне на елемент" />
</div>

Изберете `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Изберете CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Изберете CUSTOM JS/HTML" />
</div>

Сега нека отворим редактора на кода, където ще поставим нашия код.

ClickFunnels е малко объркващ в следващата стъпка.

Важно е да *НЕ* избирате `Code` когато задържите курсора върху новия елемент. Вместо това изберете `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Изберете SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Изберете SETTINGS" />
</div>

Сега от дясната страна можем да кликнем `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Кликнете Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Кликнете Open Code Editor" />
</div>

Ще видите голямо поле да се отвори. Тук можем да поставим нашия код. Копирайте следния фрагмент (използвайте бутона за копиране в горния десен ъгъл):

[inline-code-attrs-start title = 'Кодов фрагмент за поточен чат на ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // някои доставчици променят фрагмента от код да бъде async
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

Този фрагмент от код е за нашия продукт Streaming Chat, който върви добре с видеа. Ако вместо това искате фрагмента за уиджета Live Commenting, който пасва най-добре на обикновени страници или блог постове, той е в края на този урок.

Когато поставим фрагмента от код в прозореца, трябва да изглежда така:

<div class="screenshot white-bg">
    <div class="title">Поставяне на код</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Поставяне на код" />
</div>

Сега просто трябва да затворим полето:

<div class="screenshot white-bg">
    <div class="title">Затвори</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Затвори" />
</div>

Сега можете да прегледате промените си! Чувствайте се свободни да преместите уиджета и да видите къде ви харесва най-много.

<div class="screenshot white-bg">
    <div class="title">Преглед</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Преглед" />
</div>

Успех! Не забравяйте да тествате на мобилни устройства!

<div class="screenshot white-bg">
    <div class="title">Успех!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Успех!" />
</div>