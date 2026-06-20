## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postIds | array | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/user_reacts_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_user_reacts_public Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.user_reacts_response import UserReactsResponse
from client.rest import ApiException
from pprint import pprint

# Angivelse af værten er valgfri og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_user_reacts_public(tenant_id, post_ids=post_ids, sso=sso)
        print("The response of PublicApi->get_user_reacts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_reacts_public: %s\n" % e)
[inline-code-end]