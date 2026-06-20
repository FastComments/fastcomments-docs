Popis stranica za tenant. Koristi ga FChat desktop klijent za popunjavanje svoje liste soba.
Zahtijeva `enableFChat` da bude true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema pristupu grupama korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni pokazivač paginacije koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Opcionalni filtar prefiksa naslova koji nije osjetljiv na velika/mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraćaju se samo stranice koje imaju barem jedan komentar. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Primjer

[inline-code-attrs-start title = 'get_pages_public Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
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
    cursor = 'cursor_example' # str | Neprozirni pokazivač paginacije koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. (neobavezno)
    limit = 56 # int | 1..200, zadano 50 (neobavezno)
    q = 'q_example' # str | Opcionalni filtar prefiksa naslova koji nije osjetljiv na velika/mala slova. (neobavezno)
    sort_by = client.PagesSortBy() # PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). (neobavezno)
    has_comments = True # bool | Ako je true, vraćaju se samo stranice s barem jednim komentarom. (neobavezno)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]