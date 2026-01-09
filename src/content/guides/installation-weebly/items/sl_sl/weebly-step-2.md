Da bo integracija Weebly in FastComments delovala pravilno, moramo dodati **dve** majhni kosi kode.

Prvi vstavek je za skrivanje Weebly sporočila "Comments are Closed", drugi pa dejansko naloži FastComments.

Najprej kopirajte ta majhen vstavek kode:

[inline-code-attrs-start title = 'Vstavek kode FastComments (glava)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Nato, na isti strani z nastavitvami iz `Step One`, kliknite `+` poleg `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Odprite kodo glave objave</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Odprite kodo glave objave" />
</div>

Prikazal se bo besedilni okvir, kot je ta:

<div class="screenshot white-bg">
    <div class="title">Koda glave objave odprta</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Koda glave objave odprta" />
</div>

Zdaj prilepimo naš vstavek kode:

<div class="screenshot white-bg">
    <div class="title">Vstavek kode glave prilepljen</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Vstavek kode glave prilepljen" />
</div>

Sledi koda v nogi, ki omogoči FastComments. Kliknite znak plus poleg `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Odprite kodo noge objave</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Odprite kodo noge objave" />
</div>

Kopirajte ta vstavek kode, ki je zasnovan **posebej za Weebly**:

[inline-code-attrs-start title = 'Vstavek kode FastComments (noga)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // remove show comments button
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

Zdaj prilepimo našo kodo v nogi:

<div class="screenshot white-bg">
    <div class="title">Koda noge objave dodana</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Koda noge objave dodana" />
</div>

To je vse!

---