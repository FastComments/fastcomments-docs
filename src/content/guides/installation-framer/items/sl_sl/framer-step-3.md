Spodaj je FastComments izrezek za Framer Live Comments.

[inline-code-attrs-start title = 'FastComments Framer-specifičen delček za komentarje'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // nekateri ponudniki spremenijo delček kode v asinhrono
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

Ali pa lahko alternativno uporabite vdelani klepet v živo (Streaming Chat). FastComments izrezek za Framer Streaming Chat je:

[inline-code-attrs-start title = 'FastComments Framer-specifičen delček klepeta v živo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // nekateri ponudniki spremenijo delček kode v asinhrono
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

FastComments podpira urejevalnik Framer, tako da bi morali videti nekaj takega, ko prilepite kodo (morda boste morali klikniti `Publish`):

<div class="screenshot white-bg">
    <div class="title">Predogled gradnika komentarjev</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Predogled gradnika komentarjev" />
</div>

Zdaj, ko si ogledate svojo stran, bi morali videti območje za komentarje! V stranskem meniju Framer lahko po želji nastavite tudi, da je gradnik polne širine.

Upoštevajte, da Framer omejuje višino gradnikov in ne podpira samodejnega prilagajanja velikosti, zato smo tukaj izbrali gradnik Klepeta v živo, ker ima fiksno višino.