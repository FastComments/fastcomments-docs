---
## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_count_response.py)

## Primjer

[inline-code-attrs-start title = 'get_counts Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_count_response import GetBannedUsersCountResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otvorite kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvorite instancu API klase
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.get_counts(sso=sso)
        print("The response of ModerationApi->get_counts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_counts: %s\n" % e)
[inline-code-end]

---