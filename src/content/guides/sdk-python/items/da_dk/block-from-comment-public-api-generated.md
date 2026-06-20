## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`BlockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/block_success.py)

## Eksempel

[inline-code-attrs-start title = 'block_from_comment_public Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.block_success import BlockSuccess
from client.models.public_block_from_comment_params import PublicBlockFromCommentParams
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    public_block_from_comment_params = client.PublicBlockFromCommentParams() # PublicBlockFromCommentParams | 
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso=sso)
        print("The response of PublicApi->block_from_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->block_from_comment_public: %s\n" % e)
[inline-code-end]