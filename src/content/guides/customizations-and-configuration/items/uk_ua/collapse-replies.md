[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

За замовчуванням відповіді на коментарі верхнього рівня показуються.

Це можна налаштувати так, щоб користувач мав натиснути "Показати відповіді" на коментарях верхнього рівня, щоб побачити дочірні коментарі.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Це можна налаштувати без коду, на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Ця настройка не вплине на кількість початково завантажених коментарів верхнього рівня. Якщо у вас є один коментар верхнього рівня, і 29 дочірніх, з увімкненою цією опцією ви:

- Побачите коментар верхнього рівня.
- Побачите "Показати відповіді (29)" під цим коментарем.

Якщо ви хочете показувати всі коментарі верхнього рівня разом із цією опцією, встановіть [стартову сторінку на -1](#starting-page).

---