### Uso de APIs autenticadas (DefaultApi)

**Importante:** Debes establecer tu clave de API en la Configuration antes de realizar solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Crear y configurar el cliente de API
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBLIGATORIO: Establece tu clave de API (obténla desde tu panel de FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Crea la instancia de la API con el cliente configurado
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Ahora puedes hacer llamadas de API autenticadas
try:
    # Ejemplo: Agregar un usuario SSO
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
    # Errores comunes:
    # - 401: la clave API falta o no es válida
    # - 400: la validación de la solicitud falló
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

### Uso de SSO (Inicio de sesión único)

El SDK incluye utilidades para generar tokens SSO seguros:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Crear datos del usuario
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Crear instancia SSO con tu secreto de API
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generar el token SSO
sso_token = sso.create_token()

# Usa este token en tu frontend o pásalo a llamadas a la API
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

1. **Error 401 "missing-api-key"**: Asegúrate de establecer `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` antes de crear la instancia DefaultApi.
2. **Clase de API incorrecta**: Usa `DefaultApi` para solicitudes autenticadas del lado del servidor, `PublicApi` para solicitudes del lado del cliente/públicas.
3. **Errores de importación**: Asegúrate de importar desde el módulo correcto:
   - Cliente de API: `from client import ...`
   - Utilidades SSO: `from sso import ...`