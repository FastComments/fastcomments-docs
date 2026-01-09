Für Seitenreaktionen müssen wir zwei Dinge entscheiden:

- Welche Reaktionsbilder verwendet werden sollen.
- Eine kurze `id`, um jede Reaktion zu benennen.

Optional:

- Sie können außerdem optionale separate Bilder für ausgewählte/nicht ausgewählte Reaktionen definieren.
- Sie können entscheiden, ob Sie die Liste der Nutzer anzeigen möchten, die reagiert haben, wenn Sie mit der Maus über eine der Reaktionen fahren. 

[inline-code-attrs-start title = 'Codebeispiel für Seitenreaktionen'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Die Konfiguration für die React-, Angular- und andere Frontend-Bibliotheken ist dieselbe.

---