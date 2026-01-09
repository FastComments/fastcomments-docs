Next we're going to add the FastComments widget code to your site. This code will search for all forms with the title `FastComments Goes Here` and
replace it with FastComments.

So let's go to `Settings` in the bottom left of the site editor:

<div class="screenshot white-bg">
    <div class="title">Ouvrir les paramètres</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Ouvrir les paramètres" />
</div>

Open the `Custom Head Code` section:

<div class="screenshot white-bg">
    <div class="title">Ouvrir Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Ouvrir Custom Head Code" />
</div>

For Ionos we need a **special version** of the FastComments widget code. Code snippets from **other tutorials will not work.**

Now copy the following code:

[inline-code-attrs-start title = 'Extrait FastComments pour Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // on récupère l'élément qui n'est pas en pleine largeur
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...and paste it in as shown:

<div class="screenshot white-bg">
    <div class="title">Coller et enregistrer</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Coller et enregistrer" />
</div>