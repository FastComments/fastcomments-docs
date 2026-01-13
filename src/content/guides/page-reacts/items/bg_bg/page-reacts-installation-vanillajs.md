За реакциите на страницата трябва да решим две неща:

- Кои изображения за реакциите да използваме.
- Кратък `id`, за да именуваме всяка реакция.

По избор:

- Можете също да зададете по избор отделни изображения за избрани/неизбрани реакции.
- Можете да решите дали да показвате списъка с потребители, които са реагирали, когато преместите мишката върху една от реакциите. 

[inline-code-attrs-start title = 'Примерен код за реакции на страницата'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфигурацията за фронтенд библиотеките като React, Angular и т.н. е същата.

---