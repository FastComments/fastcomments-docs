### Using Authenticated APIs (DefaultApi)

**Important:** You must set your API key on the Configuration before making authenticated requests. If you don't, requests will fail with a 401 error.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Crear y configurar el cliente API
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUERIDO: Establezca su clave API (obtenga esto en su panel de FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Crear la instancia API con el cliente configurado
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Ahora puede hacer llamadas API autenticadas
try:
    # Ejemplo: Añadir un usuario SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Errores comunes:
    # - 401: Falta la clave API o es inválida
    # - 400: La validación de la solicitud falló
```

### Using Public APIs (PublicApi)

Public endpoints don't require authentication:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using the Moderation Dashboard (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Methods are called on behalf of a moderator by passing an `sso` token:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Contar los comentarios pendientes de moderación
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

The SDK includes utilities for generating secure SSO tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Crear datos de usuario
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Crear instancia SSO con su secreto API
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generar el token SSO
sso_token = sso.create_token()

# Use este token en su frontend o páselo a las llamadas API
print(f"SSO Token: {sso_token}")
```

For simple SSO (less secure, for testing):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Common Issues

1. **401 "missing-api-key" error**: Asegúrese de establecer `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` antes de crear la instancia DefaultApi.
2. **Wrong API class**: Use `DefaultApi` para solicitudes autenticadas del lado del servidor, `PublicApi` para solicitudes del lado del cliente/públicas, y `ModerationApi` para solicitudes del panel de moderador.
3. **Import errors**: Asegúrese de estar importando del módulo correcto:
   - Cliente API: `from client import ...`
   - Utilidades SSO: `from sso import ...`