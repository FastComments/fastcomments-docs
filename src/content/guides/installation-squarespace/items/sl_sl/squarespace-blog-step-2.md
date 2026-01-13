Zdaj lahko kopiramo naslednji izsek kode (uporabite gumb za kopiranje v zgornjem desnem kotu izseka):

[inline-code-attrs-start title = 'Koda komentarjev za Squarespace blog'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ID vašega računa

        function tryLoad() {
            // poskusi naložiti za različne postavitve
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

...potem prilepite v območje kode in kliknite Shrani:

<div class="screenshot white-bg">
    <div class="title">Prilepi in shrani</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Prilepi in shrani" />
</div>

---