[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments відображає бейджі користувачів лише на їхніх коментарях у потоці коментарів.

Однак ми можемо показувати бейджі користувачів поруч із їхнім ім'ям над формою коментаря, увімкнувши цю опцію на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Це відображатиме бейджі користувача поруч із його ім'ям у верхній панелі, роблячи їхні досягнення та статус більш помітними під час написання коментаря.

Зверніть увагу, що ця функція має бути ввімкнена в інтерфейсі налаштування віджета для роботи. За потреби ви можете встановити прапорець **showBadgesInTopBar** у значення false у конфігурації коду, щоб вибірково вимкнути його навіть якщо він увімкнений на рівні сервера:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---