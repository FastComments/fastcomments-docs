## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | forespørgsel | Ja |  |
| id | string | sti | Ja |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Eksempel

[inline-code-attrs-start title = 'delete_tenant_package Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere godkendelses- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode er givet nedenfor, brug det eksempel der
# opfylder dit godkendelsesscenarie.

# Konfigurer API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at indstille præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_tenant_package(tenant_id, id)
        print("The response of DefaultApi->delete_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant_package: %s\n" % e)
[inline-code-end]