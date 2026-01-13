[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Ви можете помітити, що віджет коментарів можна використовувати з Tenant ID "demo", наприклад:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Це призначено лише для ознайомлення та експериментів з віджетом коментарів. У продакшені ви передаватимете свій Tenant ID ось так:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Ваш Tenant ID вже застосовано у фрагменті коду віджета коментарів <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">у вашому обліковому записі</a>.

Ви також можете знайти свій Tenant ID та керувати своїми API-ключами [на сторінці облікових даних API](https://fastcomments.com/auth/my-account/api-secret).

Відтепер, якщо ви увійшли в FastComments, приклади коду будуть використовувати ваш реальний Tenant ID (якщо ви увійшли на https://fastcomments.com).