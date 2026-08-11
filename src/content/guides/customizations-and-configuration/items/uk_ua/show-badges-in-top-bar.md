[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments відображає значки користувачів лише у їхніх коментарях у межах потоку коментарів.

Проте ми можемо показати значки користувачів поруч із їхнім ім'ям над формою коментаря, увімкнувши цю функцію на сторінці налаштування віджету:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Show badges in top bar checkbox on the widget customization page, placing badges beside the name above the comment form'; title='Show Badges in Top Bar Option' app-screenshot-end]

Це відобразить значки користувача поруч із його ім'ям у верхній панелі, роблячи його досягнення та статус більш помітними під час написання коментаря.

Зверніть увагу, що ця функція повинна бути увімкнена в інтерфейсі налаштування віджету, щоб працювати. Ви можете за бажанням встановити прапорець **showBadgesInTopBar** у значення false у вашій конфігурації коду, щоб вибірково вимкнути його, навіть якщо він увімкнений на рівні сервера:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]