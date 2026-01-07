Добавете този фрагмент към конфигурационните файлове на AI асистента на вашия проект (като AGENT.md или CLAUDE.md), който учи AI асистентите за кодиране как да търсят и достъпват документацията на FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

По този начин вашият AI асистент може лесно да получи актуална информация и да предостави по-точни имплементации за по-малко време.

### Пример за използване

Когато използвате AI асистент като Claude Code или Cursor, можете да задавате въпроси като:

- "Как да инсталирам FastComments на React?"
- "Покажи ми как да конфигурирам SSO с FastComments"
- "Какви са опциите за автентикация на FastComments API?"

AI асистентът автоматично ще търси в документацията, използвайки предоставената API крайна точка, и ще ви даде точни, актуални отговори, базирани на официалната документация.
