Sada možemo kopirati sljedeći isječak koda (upotrijebite gumb za kopiranje u gornjem desnom kutu isječka):

[inline-code-attrs-start title = 'Kod komentara za Squarespace blog'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // id vašeg računa

        function tryLoad() {
            // pokušaj učitavanja za različite rasporede
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

...zatim zalijepite u područje koda i kliknite spremi:

<div class="screenshot white-bg">
    <div class="title">Zalijepi i spremi</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Zalijepi i spremi" />
</div>

---