Per Page Reacts dobbiamo decidere due cose:

- Quali immagini di reazione usare.
- Un breve `id` per nominare ogni reazione.

Opzionalmente:

- Puoi anche definire immagini separate opzionali per reazioni selezionate/non selezionate.
- Puoi decidere se mostrare l'elenco degli utenti che hanno reagito al passaggio del mouse su una delle reazioni. 

[inline-code-attrs-start title = 'Esempio di codice per Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

La configurazione per le librerie frontend come React, Angular e così via è la stessa.

---