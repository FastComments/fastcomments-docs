[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments дозволяє користувачам блокувати інших користувачів. Блокування користувача призведе до маскування його коментарів,
перешкоджатиме надсиланню сповіщень між користувачами тощо.

Іноді може знадобитися вимкнути цю функціональність. Це можна зробити таким чином:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Це також можна зробити без коду, що також дозволяє належну серверну валідацію, через інтерфейс налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---