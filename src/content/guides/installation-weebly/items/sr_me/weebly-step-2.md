Да би интеграција Weebly и FastComments добро функционисала, морамо додати **два** мала фрагмента кода.

Први фрагмент кода је да сакрије Weebly поруку „Коментари су затворени“, а други да заправо учита FastComments.

Прво, копирајте овај мали фрагмент кода:

[inline-code-attrs-start title = 'FastComments фрагмент кода за заглавље'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Затим, на истој страници са подешавањима из `Step One`, кликните на `+` поред `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Отворите Post Header Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Отворите Post Header Code" />
</div>

Требало би да видите поље за текст које изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Post Header Code отворено</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Post Header Code отворено" />
</div>

Сада залепите наш фрагмент кода:

<div class="screenshot white-bg">
    <div class="title">Фрагмент кода за заглавље залепљен</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Фрагмент кода за заглавље залепљен" />
</div>

Даље следи фрагмент кода за футер који омогућава FastComments. Кликните на плус поред `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Отворите Post Footer Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Отворите Post Footer Code" />
</div>

Копирајте овај фрагмент кода који је дизајниран **посебно за Weebly**:

[inline-code-attrs-start title = 'FastComments фрагмент кода за футер'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // уклони дугме за приказ коментара
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

Сада залепите наш фрагмент кода за футер:

<div class="screenshot white-bg">
    <div class="title">Post Footer Code додат</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Post Footer Code додат" />
</div>

То је то!