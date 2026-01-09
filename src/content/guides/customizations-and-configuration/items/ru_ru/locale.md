[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

По умолчанию FastComments отображает виджет комментариев в локали, определяемой системой и браузером пользователя.

Когда пользователь оставляет комментарий или выполняет вход, мы обновляем его последнюю использованную локаль и используем её также для отправки писем.

Это влияет на то, как виджет комментариев переводится для пользователя. Локаль состоит из языка и региона пользователя, поэтому настройка локали обычно меняет язык, используемый для отображения текста пользователю.

#### Via The UI

Это можно задать с помощью интерфейса настройки виджета. См. опцию "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

Это можно переопределить, указав желаемую локаль.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

[Полный список поддерживаемых языков и соответствующих кодов локалей можно найти здесь.](/guide-supported-languages.html#supported-languages)

### SSO Note

Если вы используете SSO, возможно, стоит передавать локаль пользователя в объекте user, чтобы письма и другие элементы корректно локализовывались для них.