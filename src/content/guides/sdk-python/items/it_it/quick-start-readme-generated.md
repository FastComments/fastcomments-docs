### Utilizzo delle API Autenticate (DefaultApi)

**Importante:** Devi impostare la tua chiave API nella Configuration prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Crea e configura il client API
config = Configuration()
config.host = "https://fastcomments.com/api"

# OBBLIGATORIO: Imposta la tua chiave API (ottieni questa dalla tua dashboard FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Crea l'istanza API con il client configurato
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Ora puoi effettuare chiamate API autenticate
try:
    # Esempio: Aggiungi un utente SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Errori comuni:
    # - 401: chiave API mancante o non valida
    # - 400: Convalida della richiesta fallita
```

### Utilizzo delle API Pubbliche (PublicApi)

Gli endpoint pubblici non richiedono autenticazione:

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

### Utilizzo della Dashboard di Moderazione (ModerationApi)

Il `ModerationApi` alimenta la dashboard dei moderatori. I metodi vengono chiamati per conto di un moderatore passando un token `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Conta i commenti in attesa di moderazione
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Utilizzo di SSO (Single Sign-On)

Il SDK include utilità per generare token SSO sicuri:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Crea dati utente
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Crea l'istanza SSO con il tuo segreto API
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Genera il token SSO
sso_token = sso.create_token()

# Usa questo token nel tuo frontend o passalo alle chiamate API
print(f"SSO Token: {sso_token}")
```

Per SSO semplice (meno sicuro, per test):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Problemi comuni

1. **Errore 401 "missing-api-key"**: Assicurati di impostare `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` prima di creare l'istanza DefaultApi.
2. **Classe API errata**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste lato client/pubbliche, e `ModerationApi` per richieste della dashboard dei moderatori.
3. **Errori di importazione**: Assicurati di importare dal modulo corretto:
   - Client API: `from client import ...`
   - Utilità SSO: `from sso import ...`