---
We kunnen nu het volgende codefragment kopiëren (gebruik de kopieerknop rechtsboven in het fragment):

[inline-code-attrs-start title = 'Squarespace Blogreacties-code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // uw account-id

        function tryLoad() {
            // probeer te laden voor verschillende lay-outs
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...plak het vervolgens in het codegebied en klik op Opslaan:

<div class="screenshot white-bg">
    <div class="title">Plakken en opslaan</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Plakken en opslaan" />
</div>

---