Za Page Reacts moramo odlučiti o dve stvari:

- Koje slike reakcija ćemo koristiti.
- Kratki `id` za imenovanje svake reakcije.

Opcionalno:

- Takođe možete definisati odvojene slike za izabrane/neizabrane reakcije.
- Možete odlučiti da li želite prikaz liste korisnika koji su reagovali kada se miš pređe preko jedne od reakcija. 

[inline-code-attrs-start title = 'Primer koda za Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Konfiguracija za React, Angular i druge frontend biblioteke je ista.

---