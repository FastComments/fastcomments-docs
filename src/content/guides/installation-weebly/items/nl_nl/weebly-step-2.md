Om de integratie tussen Weebly en FastComments goed te laten werken, moeten we twee kleine stukjes code toevoegen.

Het eerste fragment is om het Weebly-bericht "Comments are Closed" te verbergen, en het tweede is om daadwerkelijk FastComments te laden.

Kopieer eerst dit kleine codefragment:

[inline-code-attrs-start title = 'FastComments Koptekst Codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ga vervolgens, op dezelfde instellingenpagina als bij `Step One`, naar de `+` naast `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Open Post Header Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Open Post Header Code" />
</div>

Er zou een tekstvak moeten openen dat er zo uitziet:

<div class="screenshot white-bg">
    <div class="title">Post Header Code Open</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Post Header Code Open" />
</div>

Plak nu ons codefragment:

<div class="screenshot white-bg">
    <div class="title">Header Code Snippet Pasted</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Header Code Snippet Pasted" />
</div>

Vervolgens is er de voettekstcode om FastComments in te schakelen. Klik op het plusteken naast `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Open Post Footer Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Open Post Footer Code" />
</div>

Kopieer dit codefragment dat **speciaal voor Weebly** is ontworpen:

[inline-code-attrs-start title = 'FastComments Voettekst Codefragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // verwijder knop voor reacties weergeven
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

Plak nu onze voettekstcode:

<div class="screenshot white-bg">
    <div class="title">Post Footer Code Added</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Post Footer Code Added" />
</div>

Dat is alles!