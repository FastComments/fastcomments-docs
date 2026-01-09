Сад можемо копирати сљедећи исјечак кода. Користите дугме за копирање које се појави у горњем десном углу исјечка.

Постоји неколико ствари које можете конфигурисати у коду, погледајте линије 4 до 7.

[inline-code-attrs-start title = 'Squarespace код за појединачну страницу'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш ID налога

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Требало би да изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Залепи и сачувај</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Залепи и сачувај" />
</div>

Сада кликните Сачувај у горњем десном углу.

Имајте на уму да опција `Preview in Safe Mode` неће радити, али видгет ће се појавити када посетите ваш сајт.

Ако имате проблема, провјерите да ли при дну не пише `"tenantId": "demo"`. Требало би да буде приказан ваш tenant id ако сте пријављени. Ако није, обратите се подршци.