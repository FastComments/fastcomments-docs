Для реакцій сторінки нам потрібно вирішити дві речі:

- Які зображення реакцій використовувати.
- Короткий `id` для назви кожної реакції.

Необов'язково:

- Ви також можете визначити окремі зображення для вибраних/невибраних реакцій.
- Ви можете вирішити, чи показувати список користувачів, які відреагували, при наведенні миші на одну з реакцій. 

[inline-code-attrs-start title = 'Приклад коду для реакцій сторінки'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Конфігурація для React, Angular та інших фронтенд-бібліотек є така сама.

---