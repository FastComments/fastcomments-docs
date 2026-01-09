Pri Page Reacts se moramo odločiti o dveh zadevah:

- Katero sliko reakcije bomo uporabili.
- Kratko `id`, s katerim poimenujemo vsako reakcijo.

Neobvezno:

- Lahko določite tudi ločene neobvezne slike za izbrane in neizbrane reakcije.
- Lahko se odločite, ali želite prikazati seznam uporabnikov, ki so reagirali, ko premaknete miško nad eno izmed reakcij. 

[inline-code-attrs-start title = 'Primer kode za Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.FastCommentsUI(document.getElementById('page-reacts-example'), {
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
    });
</script>
[inline-code-end]

Konfiguracija za React, Angular in druge frontend knjižnice je enaka.

---