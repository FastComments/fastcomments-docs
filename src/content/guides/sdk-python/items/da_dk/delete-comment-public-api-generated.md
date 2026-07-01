## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| broadcastId | string | query | Ja |  |
| editKey | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_delete_comment_response.py)

## Eksempel

[inline-code-attrs-start title = 'delete_comment_public Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import DeleteCommentPublicOptions
from client.models.public_api_delete_comment_response import PublicAPIDeleteCommentResponse
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og standard til https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Indtast en kontekst med en forekomst af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en forekomst af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.delete_comment_public(tenant_id, comment_id, broadcast_id, DeleteCommentPublicOptions(edit_key=edit_key, sso=sso))
        print("The response of PublicApi->delete_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_comment_public: %s\n" % e)
[inline-code-end]