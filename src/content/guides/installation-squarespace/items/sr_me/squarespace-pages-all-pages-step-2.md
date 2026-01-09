Сада можемо копирати следећи исечак кода. Користите дугме за копирање које се појављује у горњем десном углу исечка.

Постоји неколико ствари које можете подесити у коду, погледајте линије 4 до 7.

[inline-code-attrs-start title = 'Squarespace код за коментаре на свим страницама'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш ID налога

        function tryLoad() {
            // покушајте да учитате за различите распореде
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...затим налепите у област за код и кликните сачувај. Требало би да изгледа овако, са кодом у блоку `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Налепите и сачувајте</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Налепите и сачувајте" />
</div>

Ако имате проблема, уверите се да при дну не пише `"tenantId": "demo"`. Требало би да прикаже ваш tenant id ако сте пријављени. Ако није, обратите се подршци.