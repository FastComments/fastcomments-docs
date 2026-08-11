[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

За замовчуванням відповіді на коментарі верхнього рівня відображаються.

Це можна налаштувати так, щоб користувачеві доводилося натискати "Show Replies" на коментарях верхнього рівня, щоб побачити дочірні.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Опція згортання відповідей у інтерфейсі налаштування віджета, приховуючи дочірні коментарі за посиланням Show Replies'; title='Згортання відповідей' app-screenshot-end]

Це налаштування не вплине на кількість початково завантажених коментарів верхнього рівня. Якщо у вас один коментар верхнього рівня і 29 дочірніх, при ввімкненому цьому налаштуванні ви:

- Побачите коментар верхнього рівня.
- Побачите "Show Replies" (29) під цим коментарем.

Якщо ви хочете показати всі коментарі верхнього рівня у поєднанні з цією опцією, встановіть [starting page to -1](#starting-page).

---