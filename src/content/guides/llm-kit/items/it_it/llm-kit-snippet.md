Aggiungi questo snippet ai file di configurazione dell'assistente AI del tuo progetto (come AGENT.md o CLAUDE.md) che insegna agli assistenti di codifica AI come cercare e accedere alla documentazione di FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

In questo modo il tuo assistente AI può facilmente ottenere informazioni aggiornate e fornire implementazioni più accurate in meno tempo.

### Esempio di utilizzo

Quando usi un assistente AI come Claude Code o Cursor, puoi fare domande come:

- "Come installo FastComments su React?"
- "Mostrami come configurare SSO con FastComments"
- "Quali sono le opzioni di autenticazione dell'API FastComments?"

L'assistente AI cercherà automaticamente nella documentazione utilizzando l'endpoint API fornito e ti darà risposte accurate e aggiornate basate sulla documentazione ufficiale.
