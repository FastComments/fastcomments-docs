Para Page Reacts temos que decidir duas coisas:

- Quais imagens de reação usar.
- Um `id` curto para nomear cada reação.

Opcionalmente:

- Você também pode definir imagens separadas opcionais para reações selecionadas/não selecionadas.
- Você pode decidir se deseja mostrar a lista de usuários que reagiram ao mover o mouse sobre uma das reações. 

[inline-code-attrs-start title = 'Exemplo de código de Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

A configuração para as bibliotecas front-end React, Angular, e assim por diante é a mesma.