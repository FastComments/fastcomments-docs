### Uso de APIs autenticadas (DefaultApi)

**Importante:** Debes establecer tu clave API en la Configuración antes de realizar solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

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

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
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
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uso del panel de moderación (ModerationApi)

El `ModerationApi` alimenta el panel de moderador. Los métodos se llaman en nombre de un moderador pasando un token `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uso de SSO (Single Sign-On)

El SDK incluye utilidades para generar tokens SSO seguros:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data (id, email, and username are required)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Sign it with your API secret (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generate the SSO token to pass to the widget or an API call
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

Para SSO simple (menos seguro, para pruebas):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Suscripciones en tiempo real (PubSub)

El módulo `pubsub` te permite suscribirte a eventos de comentarios en tiempo real (nuevos comentarios, votos, ediciones, notificaciones, etc.) a través de un WebSocket, replicando el `LiveEventSubscriber` del SDK Java de FastComments. Requiere el extra `pubsub`, que agrega un cliente WebSocket sobre el cliente API generado:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # e.g. a UUID for this session
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # set to "eu" for the EU region
)

# ...later, when you no longer want updates:
result.close()
```

El suscriptor ejecuta la conexión en un hilo daemon en segundo plano, se reconecta de forma transparente con jitter y recupera cualquier evento perdido mientras estaba desconectado del endpoint de registro de eventos al reconectar. Pasa un callback opcional `can_see_comments` (`List[str] -> Dict[str, str]`, que devuelve los IDs que el usuario NO puede ver) para filtrar los eventos de comentarios que el usuario no tiene permiso de ver. Establece `disable_live_commenting=True` para que `subscribe_to_changes` sea una operación nula que devuelve `None`.

### Problemas comunes

1. **Error 401 "missing-api-key"**: Asegúrate de establecer `config.api_key = {"api_key": "YOUR_KEY"}` antes de crear la instancia DefaultApi.
2 **Clase API incorrecta**: Usa `DefaultApi` para solicitudes autenticadas del lado del servidor, `PublicApi` para solicitudes del lado del cliente/públicas, y `ModerationApi` para solicitudes del panel de moderador.
3 **Errores de importación**: Asegúrate de estar importando del módulo correcto:
   - Cliente API: `from client import ...`
   - Utilidades SSO: `from sso import ...`
   - Suscripciones en tiempo real: `from pubsub import ...` (requiere el extra `pubsub`)