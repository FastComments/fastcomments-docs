[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

За замовчуванням FastComments відображає віджет коментарів у локалі, визначеній системою та браузером користувача.

Коли користувач залишає коментар або входить у систему, ми оновлюємо його останній використаний локаль і також використовуємо його для надсилання електронних листів.

Це впливає на те, як віджет коментарів буде перекладено для користувача. Locale складається з мови та регіону користувача, тому налаштування locale зазвичай змінює мову, якою відображається текст для користувача.

#### Через інтерфейс

Це можна задати за допомогою інструментів налаштування віджета. Див. опцію "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Через код

Це можна перевизначити, вказавши бажаний locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Підтримувані мови та коди локалі

[Повний список підтримуваних мов та відповідних кодів локалі можна знайти тут.](/guide-supported-languages.html#supported-languages)

### Примітка щодо SSO

Якщо ви використовуєте SSO, можливо, варто передавати locale користувача в об'єкті user, щоб електронні листи та інші елементи були правильно локалізовані для нього.

---