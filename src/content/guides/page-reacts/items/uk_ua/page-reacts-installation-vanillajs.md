Для Page Reacts нам потрібно вирішити дві речі:

- Які зображення реакцій використовувати.
- Короткий `id` для іменування кожної реакції.

Опційно:

- Ви також можете визначити окремі необов'язкові зображення для вибраних/невибраних реакцій.
- Ви можете вирішити, чи показувати список користувачів, які відреагували, при наведенні курсора миші на одну з реакцій.

[inline-code-attrs-start title = 'Приклад коду Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

Налаштування для фронтенд-бібліотек React, Angular та ін. однакове.