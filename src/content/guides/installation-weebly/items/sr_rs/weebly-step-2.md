Да би интеграција Weebly и FastComments функционисала како треба, морамо додати **два** мала фрагмента кода.

Први фрагмент је да сакрије Weebly поруку „Коментари су затворени“, а други је да заправо учита FastComments.

Прво, копирајте овај мали фрагмент кода:

[inline-code-attrs-start title = 'FastComments: фрагмент кода за заглавље'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Затим, на истој страници подешавања из `Step One`, кликните на `+` поред `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Отворите код заглавља поста</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Отворите код заглавља поста" />
</div>

Требало би да видите да се појавио текстуални оквир као на слици:

<div class="screenshot white-bg">
    <div class="title">Отворен код заглавља поста</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Отворен код заглавља поста" />
</div>

Сада налепите наш фрагмент кода:

<div class="screenshot white-bg">
    <div class="title">Фрагмент кода заглавља - налепљено</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Фрагмент кода заглавља - налепљено" />
</div>

Даље иде код футера за омогућавање FastComments. Кликните на знак плус поред `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Отворите код футера поста</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Отворите код футера поста" />
</div>

Копирајте овај фрагмент кода који је дизајниран **посебно за Weebly**:

[inline-code-attrs-start title = 'FastComments: фрагмент кода за футер'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сада налепите наш код футера:

<div class="screenshot white-bg">
    <div class="title">Код футера поста додат</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Код футера поста додат" />
</div>

То је то!