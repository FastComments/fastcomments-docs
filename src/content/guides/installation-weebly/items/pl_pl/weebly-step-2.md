Aby integracja Weebly i FastComments działała prawidłowo, musimy dodać **dwa** małe fragmenty kodu.

Pierwszy fragment ma ukryć komunikat Weebly "Comments are Closed", a drugi ma faktycznie wczytać FastComments.

Najpierw skopiuj ten mały fragment kodu:

[inline-code-attrs-start title = 'Fragment kodu nagłówka FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Następnie, na tej samej stronie ustawień z `Step One`, kliknij `+` obok `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Otwórz kod nagłówka posta</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Otwórz kod nagłówka posta" />
</div>

Powinieneś zobaczyć otwarte takie pole tekstowe:

<div class="screenshot white-bg">
    <div class="title">Kod nagłówka posta otwarty</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Kod nagłówka posta otwarty" />
</div>

Teraz wklejmy nasz fragment kodu:

<div class="screenshot white-bg">
    <div class="title">Fragment kodu nagłówka wklejony</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Fragment kodu nagłówka wklejony" />
</div>

Kolejny jest kod stopki, aby włączyć FastComments. Kliknij znak plus obok `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Otwórz kod stopki posta</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Otwórz kod stopki posta" />
</div>

Skopiuj ten fragment kodu, który został zaprojektowany **specjalnie dla Weebly**:

[inline-code-attrs-start title = 'Fragment kodu stopki FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            if (comments) { // usuń przycisk pokaż komentarze
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

Teraz wklejmy nasz kod stopki:

<div class="screenshot white-bg">
    <div class="title">Dodano kod stopki posta</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Dodano kod stopki posta" />
</div>

To wszystko!