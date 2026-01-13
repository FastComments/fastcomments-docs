[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

По подразбиране отговорите на коментарите от най-горно ниво се показват.

Това може да се конфигурира така, че потребителят да трябва да кликне върху "Покажи отговорите" при коментарите от най-горно ниво, за да види децата.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Това може да се персонализира без код, на страницата за персонализиране на уиджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Тази настройка няма да повлияе на броя на първоначално заредените коментари от най-горно ниво. Ако имате един коментар от най-горно ниво и 29 отговора, с включена тази настройка вие ще:

- Ще видите коментара от най-горно ниво.
- Ще видите "Покажи отговорите (29)" под този коментар.

Ако желаете да показвате всички коментари от най-горно ниво в комбинация с тази опция, задайте [началната страница на -1](#starting-page).