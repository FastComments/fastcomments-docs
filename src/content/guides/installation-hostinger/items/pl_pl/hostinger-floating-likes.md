FastComments obsługuje również widżet Page Reacts (znany także jako pływający przycisk Lubię to) dla Hostinger.

Możesz zobaczyć go w akcji w prawym dolnym rogu tej strony!

### Uwaga!

Te instrukcje dotyczą Kreatora stron Hostinger. Jeśli używasz Hostinger *WordPress*, po prostu skopiuj poniższy kod i dodaj go do swojej witryny WordPress, używając [WPCode](https://wordpress.org/plugins/insert-headers-and-footers/), który jest darmową i prostą w użyciu wtyczką do dodawania niewielkich fragmentów kodu na stronę.

1. Najpierw skopiuj kod:

[inline-code-attrs-start title = 'Kod pływających polubień Hostinger'; type = 'bash'; useDemoTenant = false; isFunctional = false; type = 'html';  inline-code-attrs-end]
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

2. Następnie w Hostinger otwórz kreator witryny.
3. Przejdź do Ustawień witryny w lewym dolnym rogu.
4. Wybierz Integracje.
5. Dodaj nowy kod na *koniec* pola `Custom code`, i opublikuj witrynę.
6. Nie zobaczysz widżetu w trybie podglądu, ale pojawi się on na opublikowanej wersji witryny.