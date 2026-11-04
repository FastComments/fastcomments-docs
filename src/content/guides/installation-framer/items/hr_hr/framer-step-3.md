FastComments Framer Live Comments isječak nalazi se u nastavku.

[inline-code-attrs-start title = 'FastComments Framer-Specifični Isječak Komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
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

Ili, alternativno, možete koristiti Streaming Chat widget. FastComments Framer Streaming Chat isječak je:

[inline-code-attrs-start title = 'FastComments Framer-Specifični Streaming Chat Isječak'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // some providers change the code snippet to be async
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

FastComments podržava Framer editor, tako da biste trebali vidjeti nešto poput ovoga nakon što zalijepite kod (možda ćete morati kliknuti `Publish`):

<div class="screenshot white-bg">
    <div class="title">Pregled Widgeta za Komentare</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Pregled Widgeta za Komentare" />
</div>

Sada kada pogledate svoju web stranicu, trebali biste vidjeti područje za komentare! U bočnoj traci Framer-a možete, ako želite, postaviti widget i na punu širinu.

Imajte na umu da Framer ograničava visinu widgeta i ne podržava automatsku promjenu veličine, pa smo ovdje odabrali Live Chat
widget jer ima fiksnu visinu.