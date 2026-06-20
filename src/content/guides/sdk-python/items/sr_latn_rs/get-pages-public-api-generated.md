Lista stranica za tenant. Koristi se od strane FChat desktop klijenta za popunjavanje njegove liste soba.
Zahteva da je `enableFChat` postavljeno na true u konačnoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtevaju SSO filtriraju se prema pristupu grupama korisnika koji podnosi zahtev.

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcioni filter prefiksa naslova koji ne pravi razliku između velikih i malih slova. |
| sortBy | string | query | No | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraćaju se samo stranice koje imaju bar jednim komentar. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Neproziran kursor paginacije koji se vraća kao `nextCursor` iz prethodnog zahteva. Povezan je sa istim `sortBy`. (opciono)
    limit = 56 # int | 1..200, podrazumevano 50 (opciono)
    q = 'q_example' # str | Opcioni filter prefiksa naslova koji ne pravi razliku između velikih i malih slova. (opciono)
    sort_by = client.PagesSortBy() # PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). (opciono)
    has_comments = True # bool | Ako je true, vraćaju se samo stranice koje imaju bar jedan komentar. (opciono)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]