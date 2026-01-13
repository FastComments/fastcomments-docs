За Page Reacts морамо одлучити о двије ствари:

- Које слике реакција користити.
- Кратак `id` за именовање сваке реакције.

Опционо:

- Можете такође дефинисати опционе посебне слике за одабране/неодабране реакције.
- Можете одлучити да ли желите приказати списак корисника који су реаговали када померите миша преко једне од реакција. 

[inline-code-attrs-start title = 'Пример кода за реакције странице'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфигурација за React, Angular и сличне frontend библиотеке је иста.