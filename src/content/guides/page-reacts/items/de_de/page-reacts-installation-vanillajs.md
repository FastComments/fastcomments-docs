Bei Page Reacts müssen wir zwei Dinge festlegen:

- Welche Reaktionsbilder verwendet werden sollen.
- Eine kurze `id`, um jede Reaktion zu benennen.

Optional:

- Sie können außerdem optional separate Bilder für ausgewählte und nicht ausgewählte Reaktionen definieren.
- Sie können entscheiden, ob Sie die Liste der Benutzer, die reagiert haben, beim Überfahren einer der Reaktionen mit der Maus anzeigen möchten. 

[inline-code-attrs-start title = 'Codebeispiel für Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Die Konfiguration für die Frontend-Bibliotheken wie React, Angular und so weiter ist dieselbe.

---