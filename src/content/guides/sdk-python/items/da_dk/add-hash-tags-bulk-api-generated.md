## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|-----------|--------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_create_hash_tags_response.py)

## Eksempel

[inline-code-attrs-start title = 'add_hash_tags_bulk Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.models.bulk_create_hash_tags_response import BulkCreateHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og har som standard https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere godkendelses- og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler på hver godkendelsesmetode er angivet nedenfor, brug det eksempel der
# opfylder din godkendelsesbrugssag.

# Konfigurer API-nøgle autorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Indtast en kontekst med en forekomst af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en forekomst af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (optional)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id, bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]