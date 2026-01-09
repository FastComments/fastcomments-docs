Da bi integracija Weebly i FastComments ispravno funkcionirala, moramo dodati **dva** mala isječka koda.

Prvi isječak skriva Weebly poruku "Comments are Closed", a drugi zapravo učitava FastComments.

Prvo, kopirajte ovaj mali isječak koda:

[inline-code-attrs-start title = 'Isječak koda zaglavlja FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Zatim, na istoj stranici postavki iz `Step One`, kliknite `+` pored `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Otvori kod zaglavlja posta</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Otvori kod zaglavlja posta" />
</div>

Trebali biste vidjeti otvoren tekstualni okvir ovako:

<div class="screenshot white-bg">
    <div class="title">Kod zaglavlja posta otvoren</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Kod zaglavlja posta otvoren" />
</div>

Sada zalijepimo naš isječak koda:

<div class="screenshot white-bg">
    <div class="title">Isječak koda zaglavlja zalijepljen</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Isječak koda zaglavlja zalijepljen" />
</div>

Sljedeće je kod podnožja za omogućavanje FastComments. Kliknite znak plus pored `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Otvori kod podnožja posta</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Otvori kod podnožja posta" />
</div>

Kopirajte ovaj isječak koda koji je dizajniran **posebno za Weebly**:

[inline-code-attrs-start title = 'Isječak koda podnožja FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // ukloni gumb za prikaz komentara
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

Sada zalijepimo naš kod podnožja:

<div class="screenshot white-bg">
    <div class="title">Kod podnožja posta dodan</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Kod podnožja posta dodan" />
</div>

To je to!