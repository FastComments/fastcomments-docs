Теперь, когда мы в редакторе шаблонов, нам нужно решить, где мы хотим отображать комментарии или чат в реальном времени.

В этом примере мы добавим его прямо под видео. Наведите курсор на элемент, в конец которого вы хотите добавить виджет, и нажмите `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Добавить элемент</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Добавить элемент" />
</div>

Выберите `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Выберите CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Выберите CUSTOM JS/HTML" />
</div>

Теперь давайте откроем редактор кода, куда мы вставим наш код.

На следующем шаге ClickFunnels может ввести в заблуждение.

Важно, чтобы вы *НИ В КОЕМ СЛУЧАЕ* не выбирали `Code` при наведении курсора на новый элемент. Вместо этого выберите `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Выберите SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Выберите SETTINGS" />
</div>

Теперь с правой стороны мы можем нажать `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Нажмите Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Нажмите Open Code Editor" />
</div>

Вы увидите большое окно. Здесь мы можем вставить наш код. Скопируйте следующий фрагмент (используйте кнопку копирования в правом верхнем углу):

[inline-code-attrs-start title = 'Фрагмент кода ClickFunnels для потокового чата'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // некоторые провайдеры изменяют фрагмент кода на async
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

Этот фрагмент кода предназначен для нашего продукта Streaming Chat, который хорошо подходит для видео. Если вы хотите вместо него фрагмент кода виджета Live Commenting, который лучше подходит для обычных страниц или блогов, он находится в конце этого руководства.

Когда мы вставим фрагмент кода в окно, он должен выглядеть так:

<div class="screenshot white-bg">
    <div class="title">Вставить код</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Вставить код" />
</div>

Теперь нам осталось просто закрыть окно:

<div class="screenshot white-bg">
    <div class="title">Закрыть</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Закрыть" />
</div>

Теперь вы можете просмотреть свои изменения! Не стесняйтесь перемещать виджет и посмотреть, где он вам больше нравится.

<div class="screenshot white-bg">
    <div class="title">Предпросмотр</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Предпросмотр" />
</div>

Успешно! Не забудьте протестировать мобильную версию!

<div class="screenshot white-bg">
    <div class="title">Успешно!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Успешно!" />
</div>