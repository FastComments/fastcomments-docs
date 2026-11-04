Seznam strani za najemnika. Uporablja namizni odjemalec FChat za izpolnjevanje svojega seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop do skupin zahtevajočega uporabnika.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kazalec strani, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. |
| sortBy | string | query | Ne | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). |
| hasComments | boolean | query | Ne | Če je true, vrni samo strani z vsaj enim komentarjem. |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Neprozoren kazalec strani, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. (optional)
    limit = 56 # int | 1..200, privzeto 50 (optional)
    q = 'q_example' # str | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). (optional)
    has_comments = True # bool | Če je true, vrni samo strani z vsaj enim komentarjem. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]