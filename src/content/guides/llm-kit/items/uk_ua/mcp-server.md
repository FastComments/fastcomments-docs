FastComments запускає хостинговий сервер Model Context Protocol (MCP), щоб AI-асистенти та агентні клієнти могли викликати FastComments API безпосередньо. Кожен інструмент, який надає MCP-сервер, автоматично генерується з публічної специфікації OpenAPI, тож усе, що може робити REST API, може робити й MCP-клієнт.

Ендпоінт є stateless та базується на streamable-HTTP. Немає сесії, яку потрібно підтримувати, і жодного серверного стану на клієнта.

### Ендпоінт

[inline-code-attrs-start title = 'MCP Ендпоінт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Підключення через OAuth

Будь-який MCP-клієнт, який підтримує віддалені сервери з OAuth (Claude, ChatGPT, Claude Code, Cursor та інші), може підключитися до наведеного вище ендпоінта без жодних налаштувань з боку FastComments. Клієнт реєструється через Dynamic Client Registration або ідентифікує себе за допомогою Client ID Metadata Document, відкриває браузер, щоб ви могли увійти до FastComments та схвалити доступ, і отримує токен, прив'язаний до облікового запису, у який ви увійшли.

Документи виявлення знаходяться за стандартними адресами:

[inline-code-attrs-start title = 'Виявлення'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Ваш користувач повинен мати дозвіл API Admin на обліковому записі, щоб схвалити підключення. Якщо ви керуєте кількома обліковими записами, переключіться на потрібний у панелі керування перед схваленням.

Клієнт може запросити область `read`, область `write` або обидві. Клієнт, який нічого не запитує, отримує обидві. Інструменти, що змінюють дані, не надаються токену з доступом лише для читання.

Панель керування має помічник налаштування з готовими фрагментами для вставки. Відкрийте **Integrate -> MCP Server** або відвідайте її безпосередньо:

[inline-code-attrs-start title = 'Сторінка налаштування'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Зареєструйте сервер FastComments однією командою, а потім виконайте `/mcp` у межах сесії, щоб увійти та переглянути доступні інструменти:

[inline-code-attrs-start title = 'Налаштування Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor та інші клієнти з файлами конфігурації

Додайте цей блок до конфігурації MCP-серверів вашого клієнта (`mcp.json` для Cursor). Клієнт відкриє браузер для входу при першому використанні.

[inline-code-attrs-start title = 'Конфігурація MCP-клієнта'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### Відкликання доступу

Кожне схвалене підключення перелічено в розділі **Integrate -> Connected Apps** у панелі керування. Відкликання одного з них анулює всі токени, які має цей застосунок. Застосунки реєструють себе під час підключення, і FastComments їх не перевіряє, тому відкликайте все, що вам не знайоме.

### Використання токена з REST API

Токен доступу, який отримує MCP-клієнт, є звичайним обліковим записом FastComments API. Він працює на кожному ендпоінті `/api/v1` як bearer-токен, тож застосунок, який підключився через MCP, також може викликати REST API безпосередньо:

[inline-code-attrs-start title = 'Bearer-токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Орендар визначається токеном. `tenantId` все ще можна передати, але він має збігатися. Запити `GET` потребують області `read`, а все інше — області `write`.

### Підключення за допомогою API-ключа

Клієнти, які не можуть завершити вхід через браузер, наприклад headless-сервери, можуть автентифікуватися за допомогою API-ключа. Передайте `tenantId` та `API_KEY` як параметри запиту або як HTTP-заголовки `x-tenant-id` та `x-api-key`, якщо ваш клієнт підтримує користувацькі заголовки:

[inline-code-attrs-start title = 'Ендпоінт з API-ключем'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Сторінка налаштування генерує цю URL-адресу для кожного з ваших API-ключів.

### Безпека

URL-адреса ендпоінта, що містить API-ключ, є секретом: не вставляйте її в публічні чати, скріншоти чи коміти. Якщо ключ було розкрито, змініть його на сторінці API Keys у вашій панелі керування. OAuth-токени не несуть такого ризику, оскільки вони прив'язані до одного застосунку і можуть бути відкликані через Connected Apps.