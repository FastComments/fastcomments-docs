---
## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nej |  |

## Svar

Returnerer: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Eksempel

[inline-code-attrs-start title = 'patch_hash_tag Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# At angive host er valgfrit og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentifikations- og autorisationsparametrene
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er vist nedenfor; brug eksemplet der
# passer til dit auth-brugstilfælde.

# Konfigurer API-nøgleautorisation: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tag = 'tag_example' # str | 
    tenant_id = 'tenant_id_example' # str |  (valgfri)
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (valgfri)

    try:
        api_response = api_instance.patch_hash_tag(tag, tenant_id=tenant_id, update_hash_tag_body=update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]

---