FastComments запускает хостинговый сервер Model Context Protocol (MCP), чтобы ИИ-ассистенты и агентные клиенты могли вызывать FastComments API напрямую. Каждый инструмент, который предоставляет MCP-сервер, автоматически генерируется из публичной спецификации OpenAPI, поэтому всё, что умеет REST API, доступно и MCP-клиенту.

Эндпоинт не сохраняет состояние и работает поверх streamable-HTTP. Нет сессии, которую нужно поддерживать, и нет состояния на сервере для каждого клиента.

### Эндпоинт

[inline-code-attrs-start title = 'Эндпоинт MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Подключение через OAuth

Любой MCP-клиент, поддерживающий удалённые серверы с OAuth (Claude, ChatGPT, Claude Code, Cursor и другие), может подключиться к указанному выше эндпоинту без настройки со стороны FastComments. Клиент регистрируется через Dynamic Client Registration или идентифицирует себя с помощью Client ID Metadata Document, открывает браузер, чтобы вы могли войти в FastComments и подтвердить доступ, и получает токен, привязанный к аккаунту, в который вы вошли.

Документы обнаружения находятся по стандартным адресам:

[inline-code-attrs-start title = 'Обнаружение'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Чтобы подтвердить подключение, вашему пользователю нужно разрешение API Admin на аккаунте. Если вы управляете несколькими аккаунтами, переключитесь на нужный в панели управления перед подтверждением.

Клиент может запросить скоуп `read`, скоуп `write` или оба. Клиент, который ничего не запрашивает, получает оба. Инструменты, изменяющие данные, не предоставляются токену с доступом только для чтения.

В панели управления есть помощник настройки с готовыми фрагментами для вставки. Откройте **Integrate -> MCP Server** или перейдите напрямую:

[inline-code-attrs-start title = 'Страница настройки'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Зарегистрируйте сервер FastComments одной командой, затем выполните `/mcp` внутри сессии, чтобы войти и увидеть список доступных инструментов:

[inline-code-attrs-start title = 'Настройка Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor и другие клиенты с файлом конфигурации

Добавьте этот блок в конфигурацию MCP-серверов вашего клиента (`mcp.json` для Cursor). При первом использовании клиент откроет браузер для входа.

[inline-code-attrs-start title = 'Конфигурация MCP-клиента'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Отзыв доступа

Каждое подтверждённое подключение отображается в разделе **Integrate -> Connected Apps** в панели управления. Отзыв подключения аннулирует все токены, которые есть у этого приложения. Приложения регистрируются сами при подключении, и FastComments их не проверяет, поэтому отзывайте всё, что вам не знакомо.

### Использование токена с REST API

Токен доступа, который получает MCP-клиент, — это обычный учётный данные FastComments API. Он работает на каждом эндпоинте `/api/v1` как bearer-токен, поэтому приложение, подключившееся через MCP, может также вызывать REST API напрямую:

[inline-code-attrs-start title = 'Bearer-токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Тенант определяется по токену. Параметр `tenantId` можно передать, но он должен совпадать. Запросам `GET` нужен скоуп `read`, а всем остальным — скоуп `write`.

### Подключение с API-ключом

Клиенты, которые не могут пройти вход через браузер, например headless-серверы, могут вместо этого аутентифицироваться с помощью API-ключа. Передайте `tenantId` и `API_KEY` как параметры запроса или как HTTP-заголовки `x-tenant-id` и `x-api-key`, если ваш клиент поддерживает пользовательские заголовки:

[inline-code-attrs-start title = 'Эндпоинт с API-ключом'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Страница настройки генерирует этот URL для каждого из ваших API-ключей.

### Безопасность

URL эндпоинта, содержащий API-ключ, является секретом: не вставляйте его в публичные чаты, скриншоты или коммиты. Если ключ раскрыт, смените его на странице API Keys в вашей панели управления. OAuth-токены не несут такого риска, поскольку привязаны к одному приложению и могут быть отозваны в Connected Apps.