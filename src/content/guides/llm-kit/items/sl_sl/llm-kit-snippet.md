Dodajte ta delček v konfiguracijske datoteke AI asistenta vašega projekta (kot sta AGENT.md ali CLAUDE.md), ki uči AI asistente za kodiranje, kako iskati in dostopati do dokumentacije FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Na ta način lahko vaš AI asistent enostavno pridobi posodobljene informacije in zagotovi natančnejše implementacije v krajšem času.

### Primer uporabe

Ko uporabljate AI asistenta, kot sta Claude Code ali Cursor, lahko postavljate vprašanja, kot so:

- "Kako namestiti FastComments v React?"
- "Pokaži mi, kako konfigurirati SSO s FastComments"
- "Katere so možnosti avtentikacije FastComments API?"

AI asistent bo samodejno poiskal dokumentacijo z uporabo podane API končne točke in vam dal natančne, posodobljene odgovore na podlagi uradne dokumentacije.
