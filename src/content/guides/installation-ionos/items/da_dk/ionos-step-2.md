Dernæst skal vi tilføje FastComments widget-koden til dit websted. Denne kode vil søge efter alle formularer med titlen `FastComments Goes Here` og
erstatte den med FastComments.

Så lad os gå til `Settings` i nederste venstre hjørne af site-editoren:

<div class="screenshot white-bg">
    <div class="title">Åbn Settings</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Åbn Settings" />
</div>

Åbn sektionen `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Åbn Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Åbn Custom Head Code" />
</div>

For Ionos skal vi bruge en **speciel version** af FastComments widget-koden. Kodesnippets fra **andre tutorials vil ikke virke.**

Kopiér nu følgende kode:

[inline-code-attrs-start title = 'Ionos FastComments Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // vi henter elementet, som ikke er i fuld bredde
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

...og indsæt den som vist:

<div class="screenshot white-bg">
    <div class="title">Indsæt og gem</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Indsæt og gem" />
</div>