### Using Authenticated APIs (DefaultApi)

**Important:** You must set your API key on the ApiClient before making authenticated requests. If you don't, requests will fail with a 401 error.

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

### Using Public APIs (PublicApi)

Public endpoints don't require authentication:

```ruby
require 'fastcomments'

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

### Common Issues

1. **401 "missing-api-key" error**: Make sure you set `config.api_key['x-api-key'] = 'YOUR_KEY'` before creating the DefaultApi instance.
2. **Wrong API class**: Use `DefaultApi` for server-side authenticated requests, `PublicApi` for client-side/public requests.
3. **Null API key**: The SDK will silently skip authentication if the API key is null, leading to 401 errors.