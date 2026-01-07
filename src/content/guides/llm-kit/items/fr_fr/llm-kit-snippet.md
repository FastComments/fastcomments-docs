Ajoutez cet extrait aux fichiers de configuration de l'assistant IA de votre projet (comme AGENT.md ou CLAUDE.md) qui apprend aux assistants de codage IA comment rechercher et accéder à la documentation FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

De cette façon, votre assistant IA peut facilement obtenir des informations à jour et fournir des implémentations plus précises en moins de temps.

### Exemple d'utilisation

Lors de l'utilisation d'un assistant IA comme Claude Code ou Cursor, vous pouvez poser des questions comme:

- "Comment installer FastComments sur React?"
- "Montre-moi comment configurer SSO avec FastComments"
- "Quelles sont les options d'authentification de l'API FastComments?"

L'assistant IA recherchera automatiquement la documentation en utilisant l'endpoint API fourni et vous donnera des réponses précises et à jour basées sur la documentation officielle.
