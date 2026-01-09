For at få integrationen mellem Weebly og FastComments til at fungere ordentligt, skal vi tilføje **to** små kodeudsnit.

Det første uddrag skjuler Weeblys besked "Comments are Closed", og det andet indlæser faktisk FastComments.

Først skal du kopiere dette lille kodeudsnit:

[inline-code-attrs-start title = 'FastComments Header Kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Derefter, på den samme indstillingsside fra `Step One`, klik på `+` ved siden af `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Åbn Post Header-kode</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Åbn Post Header-kode" />
</div>

Du burde se en tekstboks åbne sådan:

<div class="screenshot white-bg">
    <div class="title">Post Header-kode åbnet</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Post Header-kode åbnet" />
</div>

Nu indsætter vi vores kodeudsnit:

<div class="screenshot white-bg">
    <div class="title">Header-kodeudsnit indsat</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Header-kodeudsnit indsat" />
</div>

Næste er footerkoden for at aktivere FastComments. Klik på plus-tegnet ved siden af `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Åbn Post Footer-kode</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Åbn Post Footer-kode" />
</div>

Kopier dette kodeudsnit, som er designet **specifikt til Weebly**:

[inline-code-attrs-start title = 'FastComments Footer Kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // fjern knappen 'Vis kommentarer'
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Nu indsætter vi vores footerkode:

<div class="screenshot white-bg">
    <div class="title">Post Footer-kode tilføjet</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Post Footer-kode tilføjet" />
</div>

Det var det!