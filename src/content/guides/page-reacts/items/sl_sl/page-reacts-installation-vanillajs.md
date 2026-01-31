Za Page Reacts se moramo odločiti o dveh stvareh:

- Katere slike reakcij uporabiti.
- Kratek `id` za poimenovanje vsake reakcije.

Neobvezno:

- Lahko tudi določite ločene slike za izbrane/neizbrane reakcije.
- Lahko se odločite tudi, ali želite prikazati seznam uporabnikov, ki so reagirali, ko premaknete miško nad eno od reakcij. 

[inline-code-attrs-start title = 'Primer kode za odzive strani'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Konfiguracija za frontend knjižnice, kot so React, Angular in podobne, je enaka.