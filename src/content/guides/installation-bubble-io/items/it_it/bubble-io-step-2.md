---
Fai clic sull'elemento HTML che hai appena aggiunto. Nell'editor delle proprietà che appare, incolla il seguente codice nel campo HTML:

[inline-code-attrs-start title = 'Snippet di codice per commenti live di Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble tende a cambiare il frammento di codice rendendolo async
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
    <div class="title">Inserisci il codice FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Inserimento del codice FastComments nell'elemento HTML" />
</div>

Nota: Questo codice include un meccanismo di ritentativo per garantire che FastComments si carichi correttamente nell'ambiente dinamico di Bubble. Altri frammenti di codice non funzioneranno.

Ricorda di sostituire `'demo'` con il tuo reale tenant ID di FastComments dopo esserti registrato. Se hai eseguito l'accesso su FastComments.com, dovrebbe essere già sostituito.

---