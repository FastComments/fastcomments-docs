---
Nu kan vi kopiere følgende kodeudsnit (brug kopier-knappen øverst til højre i udsnittet):

[inline-code-attrs-start title = 'Squarespace Blogkommentarer-kode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // dit konto-id

        function tryLoad() {
            // forsøg at indlæse for forskellige layouts
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

...indsæt det derefter i kodeområdet og klik på gem:

<div class="screenshot white-bg">
    <div class="title">Indsæt og gem</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Indsæt og gem" />
</div>

---