### Простий SSO

```ruby
require 'fastcomments'

# Створити токен Simple SSO
user = FastComments::SSO::SimpleSSOUserData.new(
  user_id: 'user-123',
  email: 'user@example.com',
  avatar: 'https://example.com/avatar.jpg'
)

sso = FastComments::SSO::FastCommentsSSO.new_simple(user)
token = sso.create_token

puts "SSO Token: #{token}"

# Використати токен SSO для здійснення автентифікованого API‑виклику
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)
public_api = FastCommentsClient::PublicApi.new(api_client)

response = public_api.get_comments_public(
  'your-tenant-id',
  'your-page-url-id',
  sso: token
)

puts "Status: #{response}"
```

### Безпечний SSO

```ruby
require 'fastcomments'

# Створити токен Secure SSO
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

# Використати токен SSO для здійснення автентифікованого API‑виклику
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)
public_api = FastCommentsClient::PublicApi.new(api_client)

response = public_api.get_comments_public(
  'your-tenant-id',
  'your-page-url-id',
  sso: token
)

puts "Status: #{response}"
```