FastComments obsługuje także widżet Page Reacts (znany również jako pływający przycisk "Lubię to") dla Zyro.

Możesz zobaczyć go w akcji w prawym dolnym rogu tej strony!

1. Najpierw pobierz kod:

[inline-code-attrs-start title = 'Kod pływającego przycisku "Lubię to" dla Zyro'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-page-likes-floating.min.js?v=2" async></script>
<div id="fastcomments-page-likes-floating"></div>
<script>
    (function () {
        function tryLoad() {
            if (typeof window.FastCommentsEmbedPageLikesFloating === 'function') {
                window.FastCommentsEmbedPageLikesFloating(document.getElementById('fastcomments-page-likes-floating'), {
                    tenantId: "demo"
                });
            } else {
                setTimeout(tryLoad, 50);
            }
        }

        tryLoad();
    })();
</script>
[inline-code-end]

2. Następnie w Zyro otwórz edytor strony.
3. Przejdź do Ustawień witryny w lewym dolnym rogu.
4. Wybierz Integracje.
5. Dodaj nowy kod na *koniec* pola `Custom code` i opublikuj swoją witrynę.
6. Nie zobaczysz widżetu w trybie podglądu, ale pojawi się on na opublikowanej wersji witryny.