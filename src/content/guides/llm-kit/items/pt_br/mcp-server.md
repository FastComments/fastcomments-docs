FastComments executa um servidor hospedado do Model Context Protocol (MCP) para que assistentes de IA e clientes agentes possam chamar a API do FastComments diretamente. Cada ferramenta que o servidor MCP expõe é gerada automaticamente a partir da especificação OpenAPI pública, portanto tudo o que a API REST pode fazer, um cliente MCP pode fazer.

O endpoint é sem estado e baseado em HTTP transmissível. Não há sessão para manter ativa e nenhum estado no servidor por cliente.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Conectar com OAuth

Qualquer cliente MCP que suporte servidores remotos com OAuth (Claude, ChatGPT, Claude Code, Cursor e outros) pode conectar ao endpoint acima sem configuração no lado do FastComments. O cliente registra-se através do Registro Dinâmico de Cliente ou identifica-se com um Documento de Metadados de ID de Cliente, abre um navegador para que você possa fazer login no FastComments e aprovar o acesso, e recebe um token vinculado à conta na qual você estava conectado.

Os documentos de descoberta estão nos locais padrão:

[inline-code-attrs-start title = 'Descoberta'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Seu usuário precisa da permissão de Administrador de API na conta para aprovar uma conexão. Se você gerencia várias contas, troque para a correta no painel antes de aprovar.

Um cliente pode solicitar o escopo `read`, o escopo `write` ou ambos. Um cliente que não solicita nada recebe ambos. Ferramentas que alteram dados não são oferecidas a um token somente de leitura.

O painel possui um assistente de configuração com trechos prontos para colar. Abra **Integrar -> Servidor MCP**, ou acesse diretamente:

[inline-code-attrs-start title = 'Página de Configuração'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registre o servidor FastComments com um único comando, então execute `/mcp` dentro de uma sessão para fazer login e listar as ferramentas disponíveis:

[inline-code-attrs-start title = 'Configuração Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor e outros clientes de arquivo de configuração

Adicione este bloco à configuração de servidores MCP do seu cliente (`mcp.json` para Cursor). O cliente abre um navegador para fazer login na primeira utilização.

[inline-code-attrs-start title = 'Configuração do Cliente MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### Revogando acesso

Toda conexão aprovada é listada em **Integrar -> Aplicativos Conectados** no painel. Revogar uma invalida todos os tokens que a aplicação possui. As aplicações se registram ao conectar e o FastComments não as revisa, portanto revogue tudo o que você não reconheça.

### Usando o token com a API REST

O token de acesso que um cliente MCP obtém é uma credencial regular da API FastComments. Ele funciona em todos os endpoints `/api/v1` como um token bearer, portanto uma aplicação que se conectou via MCP também pode chamar a API REST diretamente:

[inline-code-attrs-start title = 'Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

O locatário (tenant) é implícito pelo token. Um `tenantId` ainda pode ser passado, mas deve coincidir. Requisições `GET` precisam do escopo `read` e todo o resto precisa do escopo `write`.

### Conectar com uma chave de API

Clientes que não podem concluir um login via navegador, como servidores sem interface, podem autenticar-se com uma chave de API. Passe `tenantId` e `API_KEY` como parâmetros de consulta, ou como os cabeçalhos HTTP `x-tenant-id` e `x-api-key` se seu cliente suportar cabeçalhos personalizados:

[inline-code-attrs-start title = 'Endpoint de Chave de API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

A página de configuração gera esta URL para cada uma de suas chaves de API.

### Segurança

Uma URL de endpoint que contém uma chave de API é um segredo: não cole‑a em chats públicos, capturas de tela ou commits. Se uma chave for exposta, rotacione‑a na página de Chaves de API no seu painel. Tokens OAuth não apresentam esse risco porque são vinculados a uma única aplicação e podem ser revogados em Aplicativos Conectados.

---