[related-parameter-start name = 'mentionGroupIds'; type = 'Array<string>'; related-parameter-end]

Список id для використання в автозаповненні **@mentions**. Корисно, коли потрібно запобігти відміченню користувачів, які не мають спільних груп.

Коли вказано, в автозаповненні після введення символу `@` будуть показані лише користувачі з інших груп.

[code-example-start config = {mentionGroupIds: ['yxZAhjzda', 'QT19nXbqB']}; linesToHighlight = [6, 7, 8, 9]; title = 'Limit Groups for Mentions'; code-example-end]