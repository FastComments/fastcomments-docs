### Jednostavno SSO

```ruby
require 'fastcomments'

# Kreiraj jednostavan SSO token
user = FastComments::SSO::SimpleSSOUserData.new(
  user_id: 'user-123',
  email: 'user@example.com',
  avatar: 'https://example.com/avatar.jpg'
)

sso = FastComments::SSO::FastCommentsSSO.new_simple(user)
token = sso.create_token

puts "SSO Token: #{token}"

# Koristi SSO token da izvršiš autentifikovani API poziv
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

### Sigurni SSO

```ruby
require 'fastcomments'

# Kreiraj sigurni SSO token
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

# Koristi SSO token da izvršiš autentifikovani API poziv
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