## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nej |  |
| anonUserId | string | query | Nej |  |

## Svar

Returnerer: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på flag_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host; standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentifikations- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er angivet nedenfor; brug det eksempel som
# opfylder dit auth-brugstilfælde.

# Konfigurer API-nøgle-autorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at sætte præfiks (fx Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)

    try:
        api_response = api_instance.flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]