[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Може да забележите, че коментарният widget може да се използва с Tenant ID "demo", например:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Това е само за опитване и игра с коментарния widget. В продукция трябва да подадете вашия Tenant ID, по следния начин:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Вашият Tenant ID може да бъде намерен вече приложен в коментарния widget <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">фрагмент от кода в акаунта ви</a>.

Също така можете да намерите вашия Tenant ID и да управлявате вашите API ключове [на страницата с API идентификационни данни](https://fastcomments.com/auth/my-account/api-secret).

Оттук нататък, ако сте влезли в FastComments, примерите с код ще използват вашия реален Tenant ID (ако сте влезли на https://fastcomments.com).