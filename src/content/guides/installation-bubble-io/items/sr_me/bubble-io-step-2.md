Кликните на HTML елемент који сте управо додали. У уређивачу својстава који се појави, залепите следећи код у поље за HTML:

[inline-code-attrs-start title = 'Bubble.io код за коментарисање уживо'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble обично мења исечак кода тако да постаје асинхроно
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Уметните FastComments код</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Уметање FastComments кода у HTML елемент" />
</div>

Напомена: Овај код укључује механизам поновног покушаја како би се осигурало да FastComments правилно учита у динамичном окружењу Bubble-а.
Остали исечци кода неће радити.

Запамтите да замените `'demo'` својим стварним FastComments tenant ID-ом након регистрације. Ако сте пријављени на FastComments.com, он би већ требало да буде замењен.