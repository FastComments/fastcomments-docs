## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Odziv

Returns: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## Primer

[inline-code-attrs-start title = 'get_user_badges Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetUserBadgesOptions
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Glej configuration.py za seznam vseh podprtih parametrov konfiguracije.
# Odjemalec mora nastaviti parametre za overitev in avtorizacijo
# v skladu s politiko varnosti strežnika API.
# Primeri za vsako metodo overitve so navedeni spodaj, uporabite primer,
# ki ustreza vašemu primeru uporabe overitve.

# Nastavi overitev s ključem API: api_key
# Odkomentirajte spodaj, če želite nastaviti predpono (npr. Bearer) za ključ API, po potrebi
# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvari instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (neobvezno)
    badge_id = 'badge_id_example' # str |  (neobvezno)
    type = 3.4 # float |  (neobvezno)
    displayed_on_comments = True # bool |  (neobvezno)
    limit = 3.4 # float |  (neobvezno)
    skip = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_user_badges(tenant_id, GetUserBadgesOptions(user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip))
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]