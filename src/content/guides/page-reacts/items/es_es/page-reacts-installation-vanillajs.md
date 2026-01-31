---
Para las Reacciones de página tenemos que decidir dos cosas:

- Qué imágenes de reacción usar.
- Un `id` corto para nombrar cada reacción.

Opcionalmente:

- También puedes definir imágenes separadas opcionales para reacciones seleccionadas/no seleccionadas.
- Puedes decidir si quieres mostrar la lista de usuarios que reaccionaron al pasar el cursor sobre una de las reacciones. 

[inline-code-attrs-start title = 'Ejemplo de código de Reacciones de página'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

La configuración para React, Angular y otras bibliotecas frontend es la misma.

---