Seznam strani za najemnika. Uporablja se v namiznem odjemalcu FChat za izpolnitev seznama sob.
Zahteva, da je `enableFChat` nastavljeno na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, se filtrirajo glede na skupinski dostop zahtevajočega uporabnika.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kurzor za straničenje, vrnjen kot `nextCursor` iz prejšnjega zahtevka. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezni filtarski predpona naslova brez občutljivosti na velike/male črke. |
| sortBy | string | query | Ne | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej), ali `title` (po abecedi). |
| hasComments | boolean | query | Ne | Če je true, vrni samo strani z vsaj enim komentarjem. |

## Odgovor

Vrača: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Za seznam vseh podprtih konfiguracijskih parametrov glej configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Neprozoren kurzor za straničenje, vrnjen kot `nextCursor` iz prejšnjega zahtevka. Povezan z istim `sortBy`. (neobvezno)
    limit = 56 # int | 1..200, privzeto 50 (neobvezno)
    q = 'q_example' # str | Neobvezni filtarski predpona naslova brez občutljivosti na velike/male črke. (neobvezno)
    sort_by = client.PagesSortBy() # PagesSortBy | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (po abecedi). (neobvezno)
    has_comments = True # bool | Če je true, vrni samo strani z vsaj enim komentarjem. (neobvezno)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]