Teraz możemy skopiować poniższy fragment kodu. Użyj przycisku kopiowania, który pojawia się w prawym górnym rogu fragmentu.

W kodzie można skonfigurować kilka rzeczy — zobacz linie 4–7.

[inline-code-attrs-start title = 'Kod komentarzy Squarespace — wszystkie strony'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // identyfikator twojego konta

        function tryLoad() {
            // próbuj załadować dla różnych układów
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

...następnie wklej go w obszarze kodu i kliknij zapisz. Powinno to wyglądać tak, z kodem w bloku `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Wklej i zapisz</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Wklej i zapisz" />
</div>

Jeśli masz problemy, upewnij się, że na dole nie jest napisane `"tenantId": "demo"`. Powinno pokazywać identyfikator Twojego konta, jeśli jesteś zalogowany. Jeśli nie, skontaktuj się z pomocą.