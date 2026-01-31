Для Page Reacts нам нужно решить две вещи:

- Какие изображения реакций использовать.
- Короткий `id`, чтобы назвать каждую реакцию.

Опционально:

- Вы также можете определить отдельные изображения для выбранных/невыбранных реакций.
- Вы можете решить, показывать ли список пользователей, которые отреагировали, при наведении мыши на одну из реакций. 

[inline-code-attrs-start title = 'Пример кода Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфигурация для фронтенд-библиотек React, Angular и т.д. одна и та же.

---