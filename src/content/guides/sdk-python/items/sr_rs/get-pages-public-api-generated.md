List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahteva. Vezao se za isti `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcioni filter prefiksa naslova neosetljivog na veličinu slova. |
| sortBy | string | query | No | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice sa najmanje jednim komentarom. |

## Response

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definisanje host-a je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahteva. Vezao se za isti `sortBy`. (optional)
    limit = 56 # int | 1..200, podrazumevano 50 (optional)
    q = 'q_example' # str | Opcioni filter prefiksa naslova neosetljivog na veličinu slova. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). (optional)
    has_comments = True # bool | Ako je true, vraća samo stranice sa najmanje jednim komentarom. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("Ispis odgovora PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak prilikom poziva PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]