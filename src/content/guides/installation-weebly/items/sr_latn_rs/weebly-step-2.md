Da bi integracija Weebly i FastComments radila kako treba, moramo dodati **dva** mala dela koda.

Prvi isečak koda služi za sakrivanje Weebly poruke "Comments are Closed", a drugi za stvarno učitavanje FastComments.

Prvo, kopirajte ovaj mali isečak koda:

[inline-code-attrs-start title = 'FastComments Header Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zatim, na istoj stranici sa podešavanjima iz `Step One`, kliknite na `+` pored `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Otvorite Post Header Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Otvorite Post Header Code" />
</div>

Trebalo bi da se otvori tekstualni okvir poput ovog:

<div class="screenshot white-bg">
    <div class="title">Post Header Code otvoren</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Post Header Code otvoren" />
</div>

Sada nalepimo naš isečak koda:

<div class="screenshot white-bg">
    <div class="title">Isečak koda za header nalepljen</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Isečak koda za header nalepljen" />
</div>

Sledeći korak je kod za footer koji omogućava FastComments. Kliknite na znak plus pored `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Otvorite Post Footer Code</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Otvorite Post Footer Code" />
</div>

Kopirajte ovaj isečak koda koji je dizajniran **specifično za Weebly**:

[inline-code-attrs-start title = 'FastComments Footer Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // ukloni dugme za prikaz komentara
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

Sada nalepimo naš kod za footer:

<div class="screenshot white-bg">
    <div class="title">Post Footer Code dodat</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Post Footer Code dodat" />
</div>

To je sve!