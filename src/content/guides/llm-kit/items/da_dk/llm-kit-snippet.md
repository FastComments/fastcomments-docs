Tilføj denne snippet til dit projekts AI-assistent konfigurationsfiler (som AGENT.md eller CLAUDE.md), der lærer AI-kodningsassistenter, hvordan man søger og tilgår FastComments-dokumentation.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

På denne måde kan din AI-assistent nemt få opdaterede oplysninger og levere mere præcise implementeringer på kortere tid.

### Brugseksempel

Når du bruger en AI-assistent som Claude Code eller Cursor, kan du stille spørgsmål som:

- "Hvordan installerer jeg FastComments på React?"
- "Vis mig hvordan man konfigurerer SSO med FastComments"
- "Hvad er FastComments API-godkendelsesmulighederne?"

AI-assistenten vil automatisk søge i dokumentationen ved hjælp af det angivne API-endpoint og give dig præcise, opdaterede svar baseret på den officielle dokumentation.
