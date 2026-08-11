---
Follow the same steps for `localhost` as you would production. Ensure you have production domains and API Secrets setup.

First, navigate to the [Администратор на уебкуки](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Управление на данни -> Уебкуки.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Страница за администриране на уебкуки с избирач на домейн и поле за URL на крайна точка за всяко събитие на коментар, плюс бутон „Изпрати тестово натоварване“'; title='Конфигурация на уебкуки'; cacheBuster = 'v3' app-screenshot-end]

In this page you can specify endpoints for each type of comment event.

For each type of event, be sure to click Send Test Payload to ensure you've set up your integration correctly. See the next section, "Testing", for details.

---