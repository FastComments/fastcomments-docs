[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Возможно, вы заметите, что виджет комментариев можно использовать с Tenant ID "demo", например:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Это предназначено только для опробования и экспериментов с виджетом комментариев. В рабочем окружении вы должны передать ваш Tenant ID следующим образом:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Ваш Tenant ID уже может быть указан в фрагменте кода виджета комментариев в вашей учетной записи: <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">фрагмент кода в вашей учетной записи</a>.

Вы также можете найти свой Tenant ID и управлять ключами API [на странице учетных данных API](https://fastcomments.com/auth/my-account/api-secret).

Начиная с этого момента, если вы вошли в FastComments, примеры кода будут использовать ваш реальный Tenant ID (если вы вошли на https://fastcomments.com).