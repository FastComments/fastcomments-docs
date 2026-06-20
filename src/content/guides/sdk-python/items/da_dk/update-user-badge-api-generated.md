## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Svar

Returnerer: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_success_response.py)

## Eksempel

[inline-code-attrs-start title = 'update_user_badge Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_success_response import APIEmptySuccessResponse
from client.models.update_user_badge_params import UpdateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentifikations- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode vises nedenfor, brug det eksempel der
# passer til dit auth-brugstilfælde.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at sætte prefix (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_user_badge_params = client.UpdateUserBadgeParams() # UpdateUserBadgeParams | 

    try:
        api_response = api_instance.update_user_badge(tenant_id, id, update_user_badge_params)
        print("The response of DefaultApi->update_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_user_badge: %s\n" % e)
[inline-code-end]

---