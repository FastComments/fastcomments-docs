Fügen Sie diesen Snippet zu den AI-Assistenten-Konfigurationsdateien Ihres Projekts hinzu (wie AGENT.md oder CLAUDE.md), der AI-Codierungs-Assistenten beibringt, wie sie die FastComments-Dokumentation suchen und darauf zugreifen können.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Auf diese Weise kann Ihr AI-Assistent leicht aktuelle Informationen abrufen und genauere Implementierungen in kürzerer Zeit liefern.

### Verwendungsbeispiel

Bei Verwendung eines AI-Assistenten wie Claude Code oder Cursor können Sie Fragen stellen wie:

- "Wie installiere ich FastComments auf React?"
- "Zeige mir, wie man SSO mit FastComments konfiguriert"
- "Was sind die FastComments API-Authentifizierungsoptionen?"

Der AI-Assistent wird automatisch die Dokumentation mit dem bereitgestellten API-Endpunkt durchsuchen und Ihnen genaue, aktuelle Antworten basierend auf der offiziellen Dokumentation geben.
