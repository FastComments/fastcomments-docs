Ispod se nalazi FastComments snippet za Framer Live Comments.

[inline-code-attrs-start title = 'FastComments Framer-specifičan snippet komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // neki provajderi menjaju snippet koda tako da bude asinhron
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

Ili, alternativno, možete koristiti Streaming Chat widget. FastComments snippet za Framer Streaming Chat je:

[inline-code-attrs-start title = 'FastComments Framer-specifičan snippet za Streaming Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // neki provajderi menjaju snippet koda tako da bude asinhron
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

FastComments podržava Framer editor, tako da biste trebali videti nešto ovako nakon što nalepite kod (možda ćete morati kliknuti `Publish`):

<div class="screenshot white-bg">
    <div class="title">Pregled widgeta za komentare</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Pregled widgeta za komentare" />
</div>

Sada kad pregledate svoj sajt trebalo bi da vidite deo za komentare! U bočnoj traci Framera možete, ako želite, postaviti widget da bude pune širine.

Imajte na umu da Framer ograničava visinu widgeta i ne podržava automatsko prilagođavanje veličine, zato smo ovde izabrali Live Chat widget jer ima fiksnu visinu.

---