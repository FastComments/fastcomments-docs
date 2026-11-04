Dohvaća stranice za najmoprimca (tenant). Koristi FChat desktop klijent za popunjavanje svog popisa soba.
Zahtijeva da `enableFChat` bude true na razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema pristupu grupi korisnika koji šalje zahtjev.

## Parametri

| Naziv | Vrsta | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Neobavezan filter prefiksa naslova, neosjetljiv na velika i mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraća samo stranice s barem jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer za get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
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
    cursor = 'cursor_example' # str | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. (optional)
    limit = 56 # int | 1..200, zadano 50 (optional)
    q = 'q_example' # str | Neobavezan filter prefiksa naslova, neosjetljiv na velika i mala slova. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno). (optional)
    has_comments = True # bool | Ako je true, vraća samo stranice s barem jednim komentarom. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]