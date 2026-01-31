За Page Reacts трябва да вземем решение по два въпроса:

- Кои изображения за реакции да използваме.
- Кратък `id`, за да наименувате всяка реакция.

По избор:

- Можете също да зададете отделни изображения за избрани/неизбрани реакции.
- Можете да решите дали да показвате списъка с потребители, които са реагирали, при преместване на мишката върху една от реакциите. 

[inline-code-attrs-start title = 'Примерен код за Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфигурацията за фронтенд библиотеките като React, Angular и т.н. е същата.