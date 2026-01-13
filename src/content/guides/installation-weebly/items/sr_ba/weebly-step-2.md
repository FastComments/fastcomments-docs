Да би интеграција између Weebly и FastComments добро функционисала, морамо додати **два** мала дијела кода.

Први исјечак служи да сакрије Weebly поруку "Comments are Closed", а други да заправо учита FastComments.

Прво, копирајте овај мали исјечак кода:

[inline-code-attrs-start title = 'FastComments - исјечак заглавља кода'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Отворите код заглавља објаве</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Отворите код заглавља објаве" />
</div>

Требало би да видите да се појавио текстуални оквир као овде:

<div class="screenshot white-bg">
    <div class="title">Отворен код заглавља објаве</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Отворен код заглавља објаве" />
</div>

Сада залепимо наш исјечак кода:

<div class="screenshot white-bg">
    <div class="title">Исјечак кода заглавља залепљен</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Исјечак кода заглавља залепљен" />
</div>

Следећи је код у футеру који омогућава FastComments. Кликните на плус поред `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Отворите код футера објаве</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Отворите код футера објаве" />
</div>

Копирајте овај исјечак кода који је дизајниран **специфично за Weebly**:

[inline-code-attrs-start title = 'FastComments - исјечак кода футера'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сада ћемо залепити наш код у футеру:

<div class="screenshot white-bg">
    <div class="title">Код футера објаве додат</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Код футера објаве додат" />
</div>

То је то!