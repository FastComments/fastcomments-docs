Per Page Reacts dobbiamo decidere due cose:

- Quali immagini di reazione utilizzare.
- Un breve `id` per nominare ogni reazione.

Opzionalmente:

- È inoltre possibile definire immagini separate opzionali per le reazioni selezionate/non selezionate.
- È possibile decidere se mostrare l'elenco degli utenti che hanno reagito quando si sposta il mouse sopra una delle reazioni. 

[inline-code-attrs-start title = 'Esempio di codice per Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

La configurazione per le librerie frontend React, Angular e simili è la stessa.

---