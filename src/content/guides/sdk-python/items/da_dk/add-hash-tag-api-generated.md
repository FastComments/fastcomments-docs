## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nej |  |

## Svar

Returnerer: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tag200_response.py)

## Eksempel

[inline-code-attrs-start title = 'add_hash_tag Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tag200_response import AddHashTag200Response
from client.models.create_hash_tag_body import CreateHashTagBody
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver autentificeringsmetode vises nedenfor; brug det eksempel der
# passer til din autentificeringssituation.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Fjern kommentar fra nedenstående for at sætte prefix (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (valgfri)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (valgfri)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]