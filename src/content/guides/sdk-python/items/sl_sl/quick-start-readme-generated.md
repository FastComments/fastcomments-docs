### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Morate nastaviti svoj API ključ v konfiguraciji, preden pošljete avtenticirane zahteve. Če tega ne storite, bodo zahteve spodletelo s kodo 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Ustvari in konfiguriraj API odjemalca
config = Configuration()
config.host = "https://fastcomments.com"

# OBVEZNO: Nastavi svoj API ključ (pridobi ga v nadzorni plošči FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Ustvari instanco API-ja s konfiguriranim odjemalcem
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Zdaj lahko izvajaš avtenticirane klice API-ja
try:
    # Primer: Dodaj SSO uporabnika
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Pogosti napaki:
    # - 401: API ključ manjka ali je neveljaven
    # - 400: Potrditev zahteve ni uspela
```

### Uporaba javnih API-jev (PublicApi)

Javni končni naslovi ne zahtevajo avtentikacije:

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

### Uporaba nadzorne plošče za moderiranje (ModerationApi)

`ModerationApi` poganja nadzorno ploščo moderatorja. Metode se kličejo v imenu moderatorja z podajanjem `sso` žetona:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Preštej komentarje, ki čakajo na moderiranje
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Uporaba SSO (Single Sign-On)

SDK vključuje pripomočke za ustvarjanje varnih SSO žetonov:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Ustvari podatke uporabnika (id, e-pošta in uporabniško ime so obvezni)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Podpiši ga s svojim API skrivnim ključem (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Ustvari SSO žeton, ki ga pošlješ v gradnik ali API klic
sso_token = sso.create_token()

# Uporabi ta žeton v svojem frontendu ali ga pošlji v API klice
print(f"SSO Token: {sso_token}")
```

Za preprosto SSO (manj varno, za testiranje):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Žive naročnine (PubSub)

Modul `pubsub` ti omogoča naročanje na dogodke komentarjev v realnem času (novi komentarji, glasovi, urejanja, obvestila itd.) preko WebSocket-a, kar odraža `LiveEventSubscriber` iz FastComments Java SDK. Zahteva dodatni paket `pubsub`, ki doda WebSocket odjemalca na vrh ustvarjenega API odjemalca:

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
    user_id_ws="a-unique-presence-id",  # npr. UUID za to sejo
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # nastavi na "eu" za EU regijo
)

# ...kasneje, ko ne želiš več posodobitev:
result.close()
```

Naročnik izvaja povezavo v ozadju na daemon niti, samodejno se ponovno poveže z jitterjem in pridobi morebitne manjkajoče dogodke iz končnega naslova event‑log ob ponovni povezavi. Po potrebi podajte `can_see_comments` povratni klic (`List[str] -> Dict[str, str]`, ki vrača ID‑je, ki jih uporabnik **NE** sme videti) za filtriranje dogodkov komentarjev, ki jih uporabnik ne sme videti. Nastavite `disable_live_commenting=True`, da `subscribe_to_changes` postane ne‑operativna in vrne `None`.

### Pogoste težave

1. **401 "missing-api-key" napaka**: Prepričajte se, da ste nastavili `config.api_key = {"api_key": "YOUR_KEY"}` pred ustvarjanjem instance DefaultApi.
2. **Napačen API razred**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve in `ModerationApi` za zahteve nadzorne plošče moderatorja.
3. **Napake pri uvozu**: Prepričajte se, da uvažate iz pravilnega modula:
   - API odjemalec: `from client import ...`
   - SSO pripomočki: `from sso import ...`
   - Žive naročnine: `from pubsub import ...` (potreben je dodatni paket `pubsub`)