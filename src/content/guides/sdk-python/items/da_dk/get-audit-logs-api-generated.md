## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| limit | number | query | Nej |  |
| skip | number | query | Nej |  |
| order | string | query | Nej |  |
| after | number | query | Nej |  |
| before | number | query | Nej |  |

## Svar

Returnerer: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_audit_logs Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificerings- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver autentificeringsmetode er angivet nedenfor — brug det eksempel, der opfylder dit autentificeringsbehov.

# Konfigurér API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at sætte prefix (fx Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Opret en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (valgfri)
    skip = 3.4 # float |  (valgfri)
    order = client.SORTDIR() # SORTDIR |  (valgfri)
    after = 3.4 # float |  (valgfri)
    before = 3.4 # float |  (valgfri)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]