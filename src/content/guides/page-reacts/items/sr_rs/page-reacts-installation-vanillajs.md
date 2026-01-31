За Page Reacts треба да одлучимо о две ствари:

- Које слике реакција користити.
- Кратак `id` за именовање сваке реакције.

Опционо:

- Такође можете дефинисати опционе посебне слике за селектоване/неселектоване реакције.
- Можете одлучити да ли желите да прикажете списак корисника који су реаговали када померите миша преко једне од реакција. 

[inline-code-attrs-start title = 'Пример кода за Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфигурација за React, Angular и сличне фронтенд библиотеке је иста.