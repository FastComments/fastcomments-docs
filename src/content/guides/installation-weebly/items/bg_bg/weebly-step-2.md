За да работи интеграцията между Weebly и FastComments правилно, трябва да добавим **две** малки части от код.

Първият фрагмент е за да скрием съобщението на Weebly „Comments are Closed“, а вторият е за да заредим FastComments.

Първо, копирайте този малък фрагмент от код:

[inline-code-attrs-start title = 'Фрагмент за заглавния код на FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

След това, на същата страница с настройки от `Step One`, кликнете върху `+` до `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Отворете заглавния код на публикацията</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Отворете заглавния код на публикацията" />
</div>

Трябва да видите текстово поле, което се отваря по следния начин:

<div class="screenshot white-bg">
    <div class="title">Заглавният код на публикацията е отворен</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Заглавният код на публикацията е отворен" />
</div>

Сега нека поставим нашия фрагмент от код:

<div class="screenshot white-bg">
    <div class="title">Поставен заглавен фрагмент от код</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Поставен заглавен фрагмент от код" />
</div>

Следва кодът за футъра, който активира FastComments. Кликнете върху знака плюс до `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Отворете футър кода на публикацията</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Отворете футър кода на публикацията" />
</div>

Копирайте този фрагмент от код, който е проектиран **специално за Weebly**:

[inline-code-attrs-start title = 'Фрагмент за футър кода на FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // remove show comments button
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

Сега нека поставим нашия футър код:

<div class="screenshot white-bg">
    <div class="title">Добавен е футър код на публикацията</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Добавен е футър код на публикацията" />
</div>

Това е всичко!