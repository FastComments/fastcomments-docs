W przypadku Page Reacts musimy zdecydować o dwóch rzeczach:

- Jakie obrazy reakcji użyć.
- Krótkie `id` do nazwania każdej reakcji.

Opcjonalnie:

- Możesz również zdefiniować opcjonalne osobne obrazy dla reakcji zaznaczonych/niezaznaczonych.
- Możesz zdecydować, czy chcesz pokazywać listę użytkowników, którzy zareagowali, gdy najedziesz myszką na jedną z reakcji. 

[inline-code-attrs-start title = 'Przykład kodu Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.fcConfigs = [{
        target: '#page-reacts-example',
        tenantId: 'demo',
        pageReactConfig: {
            showUsers: true,
            reacts: [
                {id: 'droll', src: 'https://docs.fastcomments.com/images/emojis/droll.png'},
                {id: 'he', src: 'https://docs.fastcomments.com/images/emojis/heart-eyes.png'},
                {id: 'laugh', src: 'https://docs.fastcomments.com/images/emojis/laugh.png'},
                {id: 'puke', src: 'https://docs.fastcomments.com/images/emojis/puke.png', selectedSrc: 'https://docs.fastcomments.com/images/emojis/puke-bw.png' },
                {id: 'rofl', src: 'https://docs.fastcomments.com/images/emojis/rofl.png' },
            ]
        }
    }];
</script>
[inline-code-end]

Konfiguracja dla bibliotek frontendowych takich jak React, Angular itp. jest taka sama.

---