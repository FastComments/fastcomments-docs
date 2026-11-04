За замовчуванням FastComments показує ім'я користувача так, як він його ввів, або так, як воно було передане нам через SSO.

Однак іноді бажано приховати або показати ім'я користувача іншим способом. Наприклад, якщо ім'я користувача — Allen Rex, можливо,
ви хочете показувати лише "Allen R.".

Це можна зробити без коду в інтерфейсі налаштування віджета, у параметрі під назвою `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Розкритий випадаючий список Commenter Name Format з варіантами, такими як Capitalize, Last Initial та All Initials'; title='Зміна формату імені' app-screenshot-end]

Доступні формати:

- Capitalize (приклад користувача відображається як Example User)
- Last Initial (Example User відображається як Example U.)
- All Initials (Example User відображається як E. U.)
- Показувати "Anonymous"

Зміна набуває чинності негайно. Користувачі, як і раніше, бачитимуть своє повне ім'я користувача вгорі області коментарів, для себе, але їхні коментарі показуватимуть
змінене ім'я користувача.

Імена користувачів маскуються на стороні сервера для захисту користувачів.