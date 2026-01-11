## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| limit | number | query | Nee |  |
| skip | number | query | Nee |  |
| order | string | query | Nee |  |
| after | number | query | Nee |  |
| before | number | query | Nee |  |

## Response

Retourneert: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_audit_logs Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs200_response import GetAuditLogs200Response
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voorbeelden voor elke auth-methode worden hieronder gegeven; gebruik het voorbeeld dat
# past bij jouw auth-gebruikssituatie.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal het commentaar hieronder weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)
    order = client.SORTDIR() # SORTDIR |  (optional)
    after = 3.4 # float |  (optional)
    before = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]