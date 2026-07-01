### Using Authenticated APIs (DefaultApi)

**Importante:** Você deve definir sua chave de API no ApiClient antes de fazer solicitações autenticadas. Caso contrário, as solicitações falharão com um erro 401.

```ruby
require 'fastcomments'

# Crie e configure o cliente da API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBRIGATÓRIO: Defina sua chave de API (obtenha isso no seu painel FastComments)
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
  # - 401: Chave de API está faltando ou é inválida
  # - 400: Validação da solicitação falhou
end
```

### Using Public APIs (PublicApi)

Os endpoints públicos não exigem autenticação:

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    'YOUR_TENANT_ID',
    'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Using Moderation APIs (ModerationApi)

Os métodos de moderação alimentam o painel do moderador. Passe um token `sso` para que a solicitação seja feita em nome de um moderador autenticado via SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Exemplo: Listar comentários na fila de moderação
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Common Issues

1. **401 "missing-api-key" error**: Certifique-se de definir `config.api_key['x-api-key'] = 'YOUR_KEY'` antes de criar a instância DefaultApi.
2. **Wrong API class**: Use `DefaultApi` para solicitações autenticadas no lado do servidor, `PublicApi` para solicitações públicas/no lado do cliente, e `ModerationApi` para solicitações do painel de moderador.
3. **Null API key**: O SDK ignorará silenciosamente a autenticação se a chave de API for nula, resultando em erros 401.