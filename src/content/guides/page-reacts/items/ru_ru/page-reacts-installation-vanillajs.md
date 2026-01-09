Для Page Reacts нам нужно решить два вопроса:

- Какие изображения реакций использовать.
- Короткий `id` для названия каждой реакции.

Опционально:

- Вы также можете задать отдельные изображения для выбранных/невыбранных реакций.
- Вы можете решить, показывать ли список пользователей, которые отреагировали, при наведении курсора мыши на одну из реакций. 

[inline-code-attrs-start title = 'Пример кода Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфигурация для фронтенд-библиотек React, Angular и т. п. одинакова.