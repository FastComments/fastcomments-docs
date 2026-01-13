Щоб інтеграція Weebly і FastComments працювала правильно, нам потрібно додати **два** невеликих фрагменти коду.

Перший фрагмент приховує повідомлення Weebly «Коментарі закрито», а другий — завантажує FastComments.

Спочатку скопіюйте цей невеликий фрагмент коду:

[inline-code-attrs-start title = 'Фрагмент коду заголовка FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Потім, на тій самій сторінці налаштувань з `Step One`, натисніть `+` поруч із `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Відкрити код заголовка допису</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Відкрити код заголовка допису" />
</div>

Ви побачите, що відкрився текстовий блок, схожий на цей:

<div class="screenshot white-bg">
    <div class="title">Відкрито код заголовка допису</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Відкрито код заголовка допису" />
</div>

Тепер вставимо наш фрагмент коду:

<div class="screenshot white-bg">
    <div class="title">Фрагмент коду заголовка вставлено</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Фрагмент коду заголовка вставлено" />
</div>

Далі додається код футера, щоб увімкнути FastComments. Натисніть на плюс поруч із `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Відкрити код футера допису</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Відкрити код футера допису" />
</div>

Скопіюйте цей фрагмент коду, розроблений **спеціально для Weebly**:

[inline-code-attrs-start title = 'Фрагмент коду футера FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // видалити кнопку показу коментарів
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

Тепер вставимо наш код футера:

<div class="screenshot white-bg">
    <div class="title">Код футера допису додано</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Код футера допису додано" />
</div>

Ось і все!