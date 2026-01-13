Voeg dit fragment toe aan de AI-assistent configuratiebestanden van je project (zoals AGENT.md of CLAUDE.md) dat AI-codeerassistenten leert hoe ze FastComments-documentatie kunnen zoeken en openen.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Op deze manier kan je AI-assistent gemakkelijk actuele informatie krijgen en nauwkeurigere implementaties in minder tijd leveren.

### Gebruiksvoorbeeld

Wanneer je een AI-assistent zoals Claude Code of Cursor gebruikt, kun je vragen stellen zoals:

- "Hoe installeer ik FastComments op React?"
- "Laat me zien hoe ik SSO met FastComments configureer"
- "Wat zijn de authenticatie-opties van de FastComments API?"

De AI-assistent zal automatisch de documentatie doorzoeken met behulp van het opgegeven API-eindpunt en je nauwkeurige, actuele antwoorden geven op basis van de officiële documentatie.
