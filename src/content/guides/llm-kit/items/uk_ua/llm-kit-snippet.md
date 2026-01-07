Додайте цей фрагмент до конфігураційних файлів AI-асистента вашого проекту (таких як AGENT.md або CLAUDE.md), який навчає AI-асистентів з програмування шукати та отримувати доступ до документації FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Таким чином ваш AI-асистент може легко отримувати актуальну інформацію та надавати точніші реалізації за менший час.

### Приклад використання

При використанні AI-асистента, такого як Claude Code або Cursor, ви можете ставити запитання на кшталт:

- "Як встановити FastComments в React?"
- "Покажи мені, як налаштувати SSO з FastComments"
- "Які є варіанти автентифікації API FastComments?"

AI-асистент автоматично виконає пошук у документації, використовуючи надану кінцеву точку API, і дасть вам точні, актуальні відповіді на основі офіційної документації.
