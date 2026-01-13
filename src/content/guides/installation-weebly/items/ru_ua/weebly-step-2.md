---
Чтобы интеграция Weebly и FastComments работала корректно, нужно добавить **два** небольших фрагмента кода.

Первый фрагмент скрывает сообщение Weebly «Comments are Closed», а второй — непосредственно загружает FastComments.

Сначала скопируйте этот небольшой фрагмент кода:

[inline-code-attrs-start title = 'Фрагмент кода заголовка FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Затем, на той же странице настроек из `Step One`, нажмите `+` рядом с `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Открыть код заголовка записи</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Открыть код заголовка записи" />
</div>

Вы увидите, что открылось текстовое поле, похожее на это:

<div class="screenshot white-bg">
    <div class="title">Код заголовка записи открыт</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Код заголовка записи открыт" />
</div>

Теперь вставим наш фрагмент кода:

<div class="screenshot white-bg">
    <div class="title">Фрагмент кода заголовка вставлен</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Фрагмент кода заголовка вставлен" />
</div>

Далее — код в футере для включения FastComments. Нажмите плюс рядом с `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Открыть код футера записи</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Открыть код футера записи" />
</div>

Скопируйте этот фрагмент кода, который разработан **специально для Weebly**:

[inline-code-attrs-start title = 'Фрагмент кода футера FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // удалить кнопку показа комментариев
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Теперь вставим код футера:

<div class="screenshot white-bg">
    <div class="title">Код футера записи добавлен</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Код футера записи добавлен" />
</div>

Готово!

---