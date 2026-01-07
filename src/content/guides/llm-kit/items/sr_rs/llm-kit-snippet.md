Додајте овај исечак у конфигурационе датотеке АИ асистента вашег пројекта (као што су AGENT.md или CLAUDE.md) који учи АИ асистенте за кодирање како да претражују и приступају документацији FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

На овај начин ваш АИ асистент може лако добити ажуриране информације и пружити тачније имплементације у краћем времену.

### Пример коришћења

Када користите АИ асистента као што је Claude Code или Cursor, можете постављати питања попут:

- "Како инсталирати FastComments на React?"
- "Покажи ми како да конфигуришем SSO са FastComments"
- "Које су опције аутентификације FastComments API-ја?"

АИ асистент ће аутоматски претражити документацију користећи наведену API крајњу тачку и дати вам тачне, ажуриране одговоре засноване на званичној документацији.
