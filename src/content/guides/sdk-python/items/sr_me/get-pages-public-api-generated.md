List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni paginacijski kurzor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan je sa istim `sortBy`. |
| limit | integer | query | No | 1..200, podrazumijevano 50 |
| q | string | query | No | Opcionalni filter prefiksa naslova bez obzira na veličinu slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (podrazumijevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (alfabetski). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice koje imaju bar jedan komentar. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'Primjer get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Neprozirni paginacijski kurzor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan je sa istim `sortBy`. (optional)
    limit = 56 # int | 1..200, podrazumijevano 50 (optional)
    q = 'q_example' # str | Opcionalni filter prefiksa naslova bez obzira na veličinu slova. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Redoslijed sortiranja. `updatedAt` (podrazumijevano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (alfabetski). (optional)
    has_comments = True # bool | Ako je true, vraća samo stranice koje imaju bar jedan komentar. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---