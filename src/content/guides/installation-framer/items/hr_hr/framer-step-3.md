Ispod se nalazi FastComments isječak za Framer Live Comments.

[inline-code-attrs-start title = 'FastComments Framer-specifični isječak komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // neki pružatelji mijenjaju isječak koda tako da je asinhron
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

Or, alternatively, you can use the Streaming Chat widget. The Framer Streaming Chat FastComments snippet is:

[inline-code-attrs-start title = 'FastComments Framer-specifični isječak za razgovor uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // neki pružatelji mijenjaju isječak koda tako da je asinhron
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
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

FastComments podržava Framer editor, pa biste trebali vidjeti nešto slično nakon što zalijepite kod (možda ćete morati kliknuti `Publish`):

<div class="screenshot white-bg">
    <div class="title">Pregled widgeta komentara</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Pregled widgeta komentara" />
</div>

Sada, kada pregledate svoju stranicu, trebali biste vidjeti područje za komentare! U bočnoj traci Framera također možete postaviti widget da bude pune širine, ako želite.

Imajte na umu da Framer ograničava visinu widgeta i ne podržava automatsko mijenjanje veličine, pa smo ovdje odabrali Live Chat widget jer ima fiksnu visinu.

---