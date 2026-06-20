## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Eksempel

[inline-code-attrs-start title = 'update_moderator Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_moderator_body import UpdateModeratorBody
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere godkendelses- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode er angivet nedenfor — brug det eksempel som
# passer til dit godkendelsesscenarie.

# Konfigurer API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentarstregen nedenfor for at sætte et præfiks (f.eks. Bearer) for API-nøglen, hvis det er nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_moderator_body = client.UpdateModeratorBody() # UpdateModeratorBody | 

    try:
        api_response = api_instance.update_moderator(tenant_id, id, update_moderator_body)
        print("The response of DefaultApi->update_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_moderator: %s\n" % e)
[inline-code-end]

---