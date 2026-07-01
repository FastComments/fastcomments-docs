## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## Eksempel

[inline-code-attrs-start title = 'add_hash_tag Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Definition af værten er valgfri og falder tilbage på https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
# Klienten skal konfigurere godkendelses- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver godkendelsesmetode er angivet nedenfor, brug det eksempel som
# opfylder dit godkendelsesbrugsscenarie.

# Konfigurer API-nøgleautorisering: api_key
# Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# Enter a context with an instance of the API client
# Opret en instans af API-klassen
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Enter a context with an instance of the API client
# Create an instance of the API class
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (optional)

    try:
        api_response = api_instance.add_hash_tag(tenant_id, create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]