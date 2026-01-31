---
Za Reakcije na stranici moramo da odlučimo o dve stvari:

- Koje slike reakcija koristiti.
- Kratak `id` za imenovanje svake reakcije.

Opcionalno:

- Možete takođe definisati opcione odvojene slike za izabrane/neuizabrane reakcije.
- Možete odlučiti da li želite da prikažete listu korisnika koji su reagovali kada se miš pomeri preko jedne od reakcija. 

[inline-code-attrs-start title = 'Primer koda za Reakcije na stranici'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Konfiguracija za React, Angular i ostale front-end biblioteke je ista.

---