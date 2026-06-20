### Uso de APIs autenticadas (DefaultApi)

**Importante:** Debes configurar tu clave de API en la Configuration antes de hacer solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### Uso de APIs públicas (PublicApi)

Los endpoints públicos no requieren autenticación:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uso del panel de moderación (ModerationApi)

La `ModerationApi` alimenta el panel de moderación. Los métodos se llaman en nombre de un moderador pasando un token `sso`:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uso de SSO (Inicio de sesión único)

El SDK incluye utilidades para generar tokens SSO seguros:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Create SSO instance with your API secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generate the SSO token
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

Para SSO simple (menos seguro, para pruebas):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Problemas comunes

1. **401 "missing-api-key" error**: Asegúrate de establecer `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` antes de crear la instancia de DefaultApi.
2. **Wrong API class**: Usa `DefaultApi` para solicitudes autenticadas en el servidor, `PublicApi` para solicitudes públicas/desde el cliente, y `ModerationApi` para solicitudes del panel de moderación.
3. **Import errors**: Asegúrate de importar desde el módulo correcto:
   - Cliente de la API: `from client import ...`
   - Utilidades de SSO: `from sso import ...`