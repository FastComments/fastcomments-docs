Добавьте этот фрагмент в конфигурационные файлы AI-ассистента вашего проекта (например, AGENT.md или CLAUDE.md), который обучает AI-ассистентов по программированию искать и получать доступ к документации FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Таким образом, ваш AI-ассистент может легко получать актуальную информацию и предоставлять более точные реализации за меньшее время.

### Пример использования

При использовании AI-ассистента, такого как Claude Code или Cursor, вы можете задавать вопросы вроде:

- "Как установить FastComments в React?"
- "Покажи мне, как настроить SSO с FastComments"
- "Какие есть варианты аутентификации API FastComments?"

AI-ассистент автоматически выполнит поиск в документации, используя предоставленную конечную точку API, и даст вам точные, актуальные ответы на основе официальной документации.
