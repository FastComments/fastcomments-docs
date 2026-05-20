FastComments executa um servidor hospedado do Protocolo de Contexto de Modelo (MCP) para que assistentes de codificação de IA e clientes agentivos possam chamar a API do FastComments diretamente. Toda ferramenta que o servidor MCP expõe é gerada automaticamente a partir da especificação pública OpenAPI, então tudo o que a API REST pode fazer, um cliente MCP também pode fazer.

O endpoint é sem estado e baseado em HTTP com streaming. Não há sessão para manter ativa, nenhum passo de registro do cliente e nenhum estado no servidor por cliente.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

A autenticação usa a mesma chave de API da API REST. Você também pode passar `tenantId` e a chave como cabeçalhos HTTP `x-tenant-id` e `x-api-key` se seu cliente suportar cabeçalhos personalizados.

### Configuração pré-preenchida

O painel tem um assistente de configuração que gera a URL e trechos de configuração prontos para colar para clientes MCP populares. Vá ao painel da sua conta e abra **Integrar -> Servidor MCP**, ou visite diretamente:

[inline-code-attrs-start title = 'Página de configuração'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Escolha qual chave de API usar no dropdown, então copie qualquer um dos trechos gerados.

### Claude Code

Registre o servidor FastComments com um comando:

[inline-code-attrs-start title = 'Configuração do Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Após o registro, execute `/mcp` dentro de uma sessão do Claude Code para confirmar a conexão e listar as ferramentas disponíveis.

### Claude Desktop / Cursor

Adicione este bloco à configuração de servidores MCP do seu cliente (`claude_desktop_config.json` para Claude Desktop, `mcp.json` para Cursor):

[inline-code-attrs-start title = 'Configuração do Cliente MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### Segurança

A chave de API está embutida na URL. Trate a URL como um segredo: não a cole em chats públicos, capturas de tela ou commits. Se uma chave for exposta, gere uma nova na página de Chaves de API no seu painel.