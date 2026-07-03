### Uso delle API Autenticate (DefaultApi)

**Importante:** Devi impostare la tua chiave API nella Configurazione prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Crea e configura il client API
config = Configuration()
config.host = "https://fastcomments.com"

# OBBLIGATORIO: Imposta la tua chiave API (ottienila dalla tua dashboard FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

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
    # - 400: convalida della richiesta fallita
```

### Uso delle API Pubbliche (PublicApi)

Gli endpoint pubblici non richiedono autenticazione:

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

### Uso della Dashboard di Moderazione (ModerationApi)

`ModerationApi` alimenta la dashboard del moderatore. I metodi sono chiamati per conto di un moderatore passando un token `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Conta i commenti in attesa di moderazione
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uso di SSO (Single Sign-On)

L'SDK include utility per generare token SSO sicuri:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Crea i dati dell'utente (id, email e username sono obbligatori)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Firma con il tuo segreto API (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Genera il token SSO da passare al widget o a una chiamata API
sso_token = sso.create_token()

# Usa questo token nel tuo frontend o passalo alle chiamate API
print(f"SSO Token: {sso_token}")
```

Per SSO semplice (meno sicuro, per test):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Problemi Comuni

1. **Errore 401 "missing-api-key"**: Assicurati di impostare `config.api_key = {"api_key": "YOUR_KEY"}` prima di creare l'istanza DefaultApi.  
2. **Classe API errata**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste client/pubbliche, e `ModerationApi` per richieste della dashboard del moderatore.  
3. **Errori di importazione**: Assicurati di importare dal modulo corretto:
   - Client API: `from client import ...`
   - Utility SSO: `from sso import ...`