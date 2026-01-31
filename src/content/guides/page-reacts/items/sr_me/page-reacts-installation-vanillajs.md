Za Page Reacts moramo odlučiti o dvije stvari:

- Koje slike reakcija koristiti.
- Kratki `id` za imenovanje svake reakcije.

Opcionalno:

- Takođe možete definirati opciona odvojena slika za odabrane/neodabrane reakcije.
- Možete odlučiti da li želite prikazati listu korisnika koji su reagovali kada pređete mišem preko neke od reakcija. 

[inline-code-attrs-start title = 'Primjer koda: Reakcije na stranici'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Konfiguracija za React, Angular i ostale frontend biblioteke je ista.

---