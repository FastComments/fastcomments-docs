List sider for en lejer. Bruges af FChat skrivebordsklienten til at udfylde sin rumliste.  
Kræver `enableFChat` at være sand på den løste brugerdefinerede konfiguration for hver side.  
Sider der kræver SSO filtreres mod den anmodende brugers gruppeadgang.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opak pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Bundet til den samme `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Valgfri case‑insensitiv titelpræfiksfilter. |
| sortBy | string | query | No | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | No | Hvis sand, returneres kun sider med mindst én kommentar. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'get_pages_public Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og har som standard https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Opak pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Bundet til den samme `sortBy`. (optional)
    limit = 56 # int | 1..200, default 50 (optional)
    q = 'q_example' # str | Valgfri case‑insensitiv titelpræfiksfilter. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk). (optional)
    has_comments = True # bool | Hvis sand, returneres kun sider med mindst én kommentar. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]