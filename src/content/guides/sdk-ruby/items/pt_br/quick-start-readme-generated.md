### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve definir sua chave de API no ApiClient antes de fazer requisições autenticadas. Se não fizer isso, as requisições falharão com um erro 401.

```ruby
require 'fastcomments'

# Create and configure the API client
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Create the API instance with the configured client
api = FastCommentsClient::DefaultApi.new(api_client)

# Now you can make authenticated API calls
begin
  # Example: Add an SSO user
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Common errors:
  # - 401: API key is missing or invalid
  # - 400: Request validation failed
end
```

### Usando APIs Públicas (PublicApi)

Endpoints públicos não requerem autenticação:

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

### Usando APIs de Moderação (ModerationApi)

Os métodos de moderação alimentam o painel do moderador. Passe um token `sso` para que a requisição seja feita em nome de um moderador autenticado via SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Example: List comments in the moderation queue
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Problemas Comuns

1. **Erro 401 "missing-api-key"**: Certifique-se de definir `config.api_key['x-api-key'] = 'YOUR_KEY'` antes de criar a instância do DefaultApi.
2. **Classe de API errada**: Use `DefaultApi` para requisições autenticadas do lado do servidor, `PublicApi` para requisições públicas/do lado do cliente, e `ModerationApi` para requisições do painel do moderador.
3. **Chave de API nula**: O SDK pulará silenciosamente a autenticação se a chave de API for nula, resultando em erros 401.