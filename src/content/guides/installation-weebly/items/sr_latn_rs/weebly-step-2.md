Da bi integracija između Weebly i FastComments radila kako treba, moramo dodati **dva** mala isječka koda.

Prvi isječak služi da sakrije Weebly poruku "Comments are Closed", a drugi da zapravo učita FastComments.

First, copy this small code snippet:

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

Then, on the same settings page from `Step One`, click the `+` next to `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Otvorite kod zaglavlja objave</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Otvorite kod zaglavlja objave" />
</div>

You should see a text box open like this:

<div class="screenshot white-bg">
    <div class="title">Kod zaglavlja objave otvoren</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Kod zaglavlja objave otvoren" />
</div>

Now let's paste in our code snippet:

<div class="screenshot white-bg">
    <div class="title">Isječak koda zaglavlja zalepljen</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Isječak koda zaglavlja zalepljen" />
</div>

Next up is the footer code to enable FastComments. Click the plus sign next to `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Otvorite kod podnožja objave</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Otvorite kod podnožja objave" />
</div>

Copy this code snippet which is designed **specifically for Weebly**:

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

Now let's paste in our footer code:

<div class="screenshot white-bg">
    <div class="title">Kod podnožja objave dodat</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Kod podnožja objave dodat" />
</div>

That's it!