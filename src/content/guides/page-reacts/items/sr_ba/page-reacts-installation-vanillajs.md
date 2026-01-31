За Page Reacts морамо одлучити о двије ствари:

- Које слике реакција ћемо користити.
- Кратак `id` да именујемо сваку реакцију.

Опционо:

- Такође можете дефинисати одвојене опционе слике за одабране и неодабране реакције.
- Можете одлучити да ли желите приказати листу корисника који су реаговали када преместите показивач миша преко једне од реакција. 

[inline-code-attrs-start title = 'Примјер кода за Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфигурација за React, Angular и остале frontend библиотеке је иста.

---