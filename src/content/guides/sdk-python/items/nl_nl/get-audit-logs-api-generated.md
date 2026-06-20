## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| limit | number | query | Nee |  |
| skip | number | query | Nee |  |
| order | string | query | Nee |  |
| after | number | query | Nee |  |
| before | number | query | Nee |  |

## Antwoord

Retourneert: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_audit_logs Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elke auth-methode zijn hieronder voorbeelden opgenomen; gebruik het voorbeeld dat
# aansluit bij uw gebruiksgeval.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal hieronder het commentaar weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (optioneel)
    skip = 3.4 # float |  (optioneel)
    order = client.SORTDIR() # SORTDIR |  (optioneel)
    after = 3.4 # float |  (optioneel)
    before = 3.4 # float |  (optioneel)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]