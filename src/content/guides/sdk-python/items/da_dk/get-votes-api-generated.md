## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |

## Respons

Returnerer: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_votes Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_response import GetVotesResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host; som standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentifikations- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er angivet nedenfor; brug det eksempel som
# passer til dit autentifikationsscenarie.

# Konfigurer API-nøgleautorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_votes(tenant_id, url_id)
        print("The response of DefaultApi->get_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes: %s\n" % e)
[inline-code-end]