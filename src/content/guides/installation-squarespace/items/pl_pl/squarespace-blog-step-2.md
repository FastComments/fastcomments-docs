---
Teraz możemy skopiować następujący fragment kodu (użyj przycisku kopiowania w prawym górnym rogu fragmentu):

[inline-code-attrs-start title = 'Kod komentarzy dla bloga Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // id twojego konta

        function tryLoad() {
            // spróbuj załadować dla różnych układów
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

...następnie wklej w obszar kodu i kliknij zapisz:

<div class="screenshot white-bg">
    <div class="title">Wklej i zapisz</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Wklej i zapisz" />
</div>

---