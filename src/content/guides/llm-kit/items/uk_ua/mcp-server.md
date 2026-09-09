FastComments запускає розміщений сервер Model Context Protocol (MCP), щоб AI‑асистенти та агентні клієнти могли безпосередньо викликати API FastComments. Кожен інструмент, який надає сервер MCP, автоматично генерується з публічної специфікації OpenAPI, тому все, що може робити REST API, може робити і клієнт MCP.

Кінцева точка є безстановою та базується на streamable‑HTTP. Не існує сесії, яку треба підтримувати, і немає серверного стану для кожного клієнта.

### Endpoint

[inline-code-attrs-start title = 'Кінцева точка MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Будь-який клієнт MCP, який підтримує віддалені сервери з OAuth (Claude, ChatGPT, Claude Code, Cursor та інші), може підключитися до вищезазначеної кінцевої точки без налаштувань зі сторони FastComments. Клієнт реєструє себе через Dynamic Client Registration або ідентифікує себе за допомогою документа Client ID Metadata, відкриває браузер, щоб ви могли ввійти в FastComments і схвалити доступ, і отримує токен, прив’язаний до облікового запису, у який ви ввійшли.

Документи відкриття розташовані у стандартних місцях:

[inline-code-attrs-start title = 'Відкриття'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Ваш користувач потребує дозволу API Admin у обліковому записі, щоб схвалити підключення. Якщо ви керуєте кількома обліковими записами, перед схваленням перемкніться на потрібний у панелі керування.

Клієнт може запитати область `read`, область `write` або обидві. Клієнт, який нічого не запитує, отримує обидві. Інструменти, які змінюють дані, не пропонуються токену лише для читання.

Панель керування має помічник налаштування з готовими до вставки фрагментами. Відкрийте **Integrate -> MCP Server** або перейдіть безпосередньо:

[inline-code-attrs-start title = 'Сторінка налаштування'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Зареєструйте сервер FastComments за допомогою однієї команди, а потім запустіть `/mcp` у сеансі, щоб ввійти та отримати список доступних інструментів:

[inline-code-attrs-start title = 'Налаштування Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Додайте цей блок до конфігурації MCP серверів вашого клієнта (`mcp.json` для Cursor). Клієнт відкриває браузер для входу при першому використанні.

[inline-code-attrs-start title = 'Конфігурація клієнта MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Revoking access

Кожне схвалене підключення відображається у розділі **Integrate -> Connected Apps** в панелі керування. Відкликання одного підключення анулює всі токени, які має ця програма. Програми реєструються самостійно під час підключення, і FastComments їх не перевіряє, тому відкликайте все, що ви не розпізнаєте.

### Using the token with the REST API

Токен доступу, який отримує клієнт MCP, є звичайним обліковим записом API FastComments. Він працює на кожній кінцевій точці `/api/v1` як токен типу bearer, тому програма, підключена через MCP, може також безпосередньо викликати REST API:

[inline-code-attrs-start title = 'Токен Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Орендар (tenant) визначається токеном. `tenantId` все ще можна передати, але він має збігатися. Запити `GET` потребують області `read`, а всі інші — області `write`.

### Connect with an API key

Клієнти, які не можуть виконати вхід у браузері, наприклад безголові сервери, можуть автентифікуватися за допомогою API ключа. Передайте `tenantId` та `API_KEY` як параметри запиту або як HTTP‑заголовки `x-tenant-id` та `x-api-key`, якщо ваш клієнт підтримує користувацькі заголовки:

[inline-code-attrs-start title = 'Кінцева точка API ключа'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Сторінка налаштування генерує це URL для кожного вашого API ключа.

### Security

URL кінцевої точки, що містить API ключ, є секретом: не вставляйте його у публічні чати, скріншоти чи коміти. Якщо ключ був розкритий, оновіть його на сторінці API Keys у вашій панелі керування. OAuth токени не несуть такого ризику, оскільки вони прив’язані до однієї програми і їх можна відкликати у розділі Connected Apps.