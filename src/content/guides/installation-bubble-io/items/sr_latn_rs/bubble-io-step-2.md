---
Kliknite na HTML element koji ste upravo dodali. U uređivaču svojstava koji se pojavi, zalepite sledeći kod u HTML polje:

[inline-code-attrs-start title = 'Bubble.io kod za komentarisanje uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // bubble obično menja isječak koda tako da postane asinhron
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
    <div class="title">Ubaci FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Ubacivanje FastComments koda u HTML element" />
</div>

Napomena: Ovaj kod uključuje mehanizam ponovnog pokušaja da bi se osiguralo da se FastComments pravilno učita u dinamičkom okruženju Bubble-a.
Ostali isječci koda neće raditi.

Ne zaboravite da zamenite `'demo'` vašim stvarnim FastComments tenant ID-jem nakon prijave. Ako ste prijavljeni na FastComments.com, trebalo bi da je već zamenjeno.

---