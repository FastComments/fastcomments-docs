Sada možemo kopirati sledeći isječak koda (koristite dugme za kopiranje u gornjem desnom uglu isječka):

[inline-code-attrs-start title = 'Squarespace kod komentara na blogu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // vaš ID naloga

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

...zatim nalepite u polje za kod i kliknite na Sačuvaj:

<div class="screenshot white-bg">
    <div class="title">Nalepite i sačuvajte</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Nalepite i sačuvajte" />
</div>