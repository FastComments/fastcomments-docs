### SSO semplice

```ruby
require 'fastcomments'
require 'fastcomments-client'

# Crea token SSO semplice
user = FastComments::SSO::SimpleSSOUserData.new(
  user_id: 'user-123',
  email: 'user@example.com',
  avatar: 'https://example.com/avatar.jpg'
)

sso = FastComments::SSO::FastCommentsSSO.new_simple(user)
token = sso.create_token

puts "SSO Token: #{token}"

# Usa il token SSO per effettuare una chiamata API autenticata
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)
public_api = FastCommentsClient::PublicApi.new(api_client)

response = public_api.get_comments_public(
  tenant_id: 'your-tenant-id',
  url_id: 'your-page-url-id',
  sso: token
)

puts "Status: #{response}"
```

### SSO sicuro

```ruby
require 'fastcomments'
require 'fastcomments-client'

# Crea token SSO sicuro
user = FastComments::SSO::SecureSSOUserData.new(
  user_id: 'user-123',
  email: 'user@example.com',
  username: 'johndoe',
  avatar: 'https://example.com/avatar.jpg'
)

api_key = 'your-api-key'
sso = FastComments::SSO::FastCommentsSSO.new_secure(api_key, user)
token = sso.create_token

puts "Secure SSO Token: #{token}"

# Usa il token SSO per effettuare una chiamata API autenticata
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)
public_api = FastCommentsClient::PublicApi.new(api_client)

response = public_api.get_comments_public(
  tenant_id: 'your-tenant-id',
  url_id: 'your-page-url-id',
  sso: token
)

puts "Status: #{response}"
```