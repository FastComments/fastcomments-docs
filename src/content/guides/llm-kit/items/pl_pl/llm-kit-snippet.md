Dodaj ten fragment do plików konfiguracyjnych asystenta AI w swoim projekcie (takich jak AGENT.md lub CLAUDE.md), który uczy asystentów kodowania AI jak wyszukiwać i uzyskiwać dostęp do dokumentacji FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

W ten sposób Twój asystent AI może łatwo uzyskać aktualne informacje i zapewnić dokładniejsze implementacje w krótszym czasie.

### Przykład użycia

Podczas korzystania z asystenta AI, takiego jak Claude Code lub Cursor, możesz zadawać pytania takie jak:

- "Jak zainstalować FastComments w React?"
- "Pokaż mi jak skonfigurować SSO z FastComments"
- "Jakie są opcje uwierzytelniania API FastComments?"

Asystent AI automatycznie przeszuka dokumentację używając podanego punktu końcowego API i udzieli Ci dokładnych, aktualnych odpowiedzi opartych na oficjalnej dokumentacji.
