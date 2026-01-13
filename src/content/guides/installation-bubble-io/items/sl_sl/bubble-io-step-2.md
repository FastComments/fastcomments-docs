Kliknite na HTML element, ki ste ga pravkar dodali. V urejevalniku lastnosti, ki se prikaže, prilepite naslednjo kodo v polje HTML:

[inline-code-attrs-start title = 'Delček kode za komentiranje v živo Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble pogosto preuredi izrezek kode v asinhrono obliko
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
    <div class="title">Vstavite kodo FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Vstavljanje kode FastComments v HTML element" />
</div>

Opomba: Ta koda vključuje mehanizem ponovnega poskusa, da se zagotovi pravilno nalaganje FastComments v dinamičnem okolju Bubble.
Drugi izrezki kode ne bodo delovali.

Ne pozabite zamenjati 'demo' z dejanskim ID-jem najemnika FastComments po prijavi. Če ste prijavljeni na FastComments.com, bi moralo biti to že zamenjano.