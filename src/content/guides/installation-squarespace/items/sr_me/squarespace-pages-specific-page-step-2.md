Сада можемо копирати следећи фрагмент кода. Користите дугме за копирање које се појављује у горњем десном углу исечка.

Постоје неке опције које можете подесити у коду, погледајте линије 4 до 7.

[inline-code-attrs-start title = 'Squarespace код за појединачну страницу'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш идентификатор налога

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

То би требало да изгледа овако:

<div class="screenshot white-bg">
    <div class="title">Paste and Save</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Paste and Save" />
</div>

Сада кликните на Сачувај у горњем десном углу.

Имајте на уму да опција `Preview in Safe Mode` неће радити, али ће виџет бити видљив када посетите ваш сајт.

Ако имате проблема, проверите да ли при дну не пише `"tenantId": "demo"`. Требало би да прикаже ваш tenant id ако сте пријављени. У супротном, обратите се подршци.