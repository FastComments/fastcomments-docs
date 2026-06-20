---
## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_count_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_counts Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_count_response import GetBannedUsersCountResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og som standard bruges https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_counts(sso=sso)
        print("The response of ModerationApi->get_counts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_counts: %s\n" % e)
[inline-code-end]

---