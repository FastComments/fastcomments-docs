Now we can copy the following code snippet (use the copy button in the top right of the snippet):

[inline-code-attrs-start title = 'Squarespace Blog Yorumları Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // hesabınızın kimliği

        function tryLoad() {
            // farklı düzenler için yüklemeyi dene
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

...then paste in the code area and click save:

<div class="screenshot white-bg">
    <div class="title">Yapıştır ve Kaydet</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Yapıştır ve Kaydet" />
</div>

---