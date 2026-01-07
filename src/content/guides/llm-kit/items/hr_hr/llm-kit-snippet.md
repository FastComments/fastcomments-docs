Dodajte ovaj isječak u konfiguracijske datoteke AI asistenta vašeg projekta (poput AGENT.md ili CLAUDE.md) koji uči AI asistente za kodiranje kako pretraživati i pristupiti dokumentaciji FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Na ovaj način vaš AI asistent može lako dobiti ažurirane informacije i pružiti točnije implementacije u kraćem vremenu.

### Primjer korištenja

Kada koristite AI asistenta poput Claude Code ili Cursor, možete postavljati pitanja poput:

- "Kako instalirati FastComments na React?"
- "Pokaži mi kako konfigurirati SSO s FastComments"
- "Koje su opcije autentifikacije FastComments API-ja?"

AI asistent će automatski pretražiti dokumentaciju koristeći navedenu API krajnju točku i dati vam točne, ažurirane odgovore temeljene na službenoj dokumentaciji.
