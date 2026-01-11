### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve definir sua chave de API no ApiClient antes de fazer requisições autenticadas. Se não fizer isso, as requisições falharão com um erro 401.

```ruby
require 'fastcomments-client'

# Crie e configure o cliente da API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBRIGATÓRIO: Defina sua chave de API (pegue isto do painel do FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Crie a instância da API com o cliente configurado
api = FastCommentsClient::DefaultApi.new(api_client)

# Agora você pode fazer chamadas de API autenticadas
begin
  # Exemplo: Adicionar um usuário SSO
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Erros comuns:
  # - 401: chave de API está faltando ou é inválida
  # - 400: validação da requisição falhou
end
```

### Usando APIs Públicas (PublicApi)

Endpoints públicos não exigem autenticação:

```ruby
require 'fastcomments-client'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    tenant_id: 'YOUR_TENANT_ID',
    url_id: 'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Problemas Comuns

1. **Erro 401 "missing-api-key"**: Certifique-se de definir `config.api_key['x-api-key'] = 'YOUR_KEY'` antes de criar a instância DefaultApi.
2. **Classe de API incorreta**: Use `DefaultApi` para requisições do lado do servidor autenticadas, `PublicApi` para requisições do lado do cliente/públicas.
3. **Chave de API nula**: O SDK ignorará silenciosamente a autenticação se a chave de API for nula, resultando em erros 401.