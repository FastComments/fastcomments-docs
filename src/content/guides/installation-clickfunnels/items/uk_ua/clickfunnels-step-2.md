Тепер, коли ми в редакторі шаблонів, потрібно вирішити, де саме ми хочемо відображати коментарі або чат в реальному часі.

У цьому прикладі ми додамо його безпосередньо під відео. Наведіть курсор на елемент, щоб додати віджет у кінець, і натисніть `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Додати елемент</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Додати елемент" />
</div>

Оберіть `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Оберіть CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Оберіть CUSTOM JS/HTML" />
</div>

Тепер відкриємо редактор коду, куди вставимо наш код.

У ClickFunnels наступний крок трохи заплутаний.

Важливо, щоб ви *НЕ* обирали `Code`, коли наведете курсор на новий елемент. Натомість оберіть `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Оберіть SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Оберіть SETTINGS" />
</div>

Тепер праворуч ми можемо натиснути `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Натисніть Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Натисніть Open Code Editor" />
</div>

Відкриється велика площина. Саме сюди ми можемо вставити наш код. Скопіюйте наступний фрагмент (використайте кнопку копіювання у верхньому правому куті):

[inline-code-attrs-start title = 'Фрагмент коду стрімінгового чату ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // деякі провайдери змінюють фрагмент коду, роблячи його async
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

Цей фрагмент коду призначений для нашого продукту Streaming Chat, який добре підходить для відео. Якщо вам потрібен фрагмент коду для віджета Live Commenting, який краще підходить для звичайних сторінок або блог-публікацій, він знаходиться в кінці цього посібника.

Коли ми вставимо фрагмент коду у вікно, це має виглядати так:

<div class="screenshot white-bg">
    <div class="title">Вставити код</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Вставити код" />
</div>

Залишається лише закрити вікно:

<div class="screenshot white-bg">
    <div class="title">Закрити</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Закрити" />
</div>

Тепер ви можете попередньо переглянути внесені зміни! Сміливо переміщуйте віджет та дивіться, де він найбільше підходить.

<div class="screenshot white-bg">
    <div class="title">Попередній перегляд</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Попередній перегляд" />
</div>

Успіх! Не забудьте протестувати мобільну версію!

<div class="screenshot white-bg">
    <div class="title">Успіх!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Успіх!" />
</div>