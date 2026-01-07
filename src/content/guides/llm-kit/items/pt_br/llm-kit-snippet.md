Adicione este trecho aos arquivos de configuração do assistente de IA do seu projeto (como AGENT.md ou CLAUDE.md) que ensina assistentes de codificação de IA como pesquisar e acessar a documentação do FastComments.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

Desta forma, seu assistente de IA pode facilmente obter informações atualizadas e fornecer implementações mais precisas em menos tempo.

### Exemplo de uso

Ao usar um assistente de IA como Claude Code ou Cursor, você pode fazer perguntas como:

- "Como instalar o FastComments no React?"
- "Mostre-me como configurar SSO com FastComments"
- "Quais são as opções de autenticação da API do FastComments?"

O assistente de IA pesquisará automaticamente a documentação usando o endpoint de API fornecido e lhe dará respostas precisas e atualizadas com base na documentação oficial.
