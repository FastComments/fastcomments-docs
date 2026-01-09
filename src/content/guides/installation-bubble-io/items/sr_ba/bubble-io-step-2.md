Kliknite na HTML element koji ste upravo dodali. U uređivaču svojstava koji se pojavi, zalijepite sljedeći kod u HTML polje:

[inline-code-attrs-start title = 'Bubble.io Kod za komentarisanje uživo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble često mijenja isječak koda i čini ga asinkronim
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
    <div class="title">Umetnite FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Umetanje FastComments koda u HTML element" />
</div>

Napomena: Ovaj kod uključuje mehanizam ponovnog pokušaja kako bi se osiguralo da se FastComments pravilno učita u dinamičkom okruženju Bubble-a.
Ostali isječci koda neće raditi.

Zapamtite da zamijenite `'demo'` sa stvarnim FastComments tenant ID-jem nakon registracije. Ako ste prijavljeni na FastComments.com, trebalo bi da je već zamijenjeno.