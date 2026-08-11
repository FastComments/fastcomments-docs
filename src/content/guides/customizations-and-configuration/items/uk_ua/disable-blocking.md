[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments дозволяє користувачам блокувати інших користувачів. Блокування користувача призведе до маскування їхніх коментарів, запобігає сповіщенням між користувачами тощо.

Можливо, буде бажано вимкнути цю функціональність. Це можна зробити так:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Вимкнути блокування'; code-example-end]

Це також можна зробити без коду, що також забезпечує правильну серверну валідацію, за допомогою інтерфейсу налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Опція вимкнення блокування в інтерфейсі налаштування віджета, яка запобігає блокуванню користувачами один одного'; title='Вимкнути блокування' app-screenshot-end]