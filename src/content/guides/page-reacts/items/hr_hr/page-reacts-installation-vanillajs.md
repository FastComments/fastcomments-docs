Za reakcije na stranici moramo odlučiti o dvije stvari:

- Koje slike reakcija koristiti.
- Kratki `id` za imenovanje svake reakcije.

Opcionalno:

- Također možete definirati odvojene slike za odabrane/neodabrane reakcije.
- Možete odlučiti želite li prikazati popis korisnika koji su reagirali kada se miš premakne preko jedne od reakcija. 

[inline-code-attrs-start title = 'Primjer koda za reakcije stranice'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Konfiguracija za React, Angular i ostale frontend biblioteke je ista.

---