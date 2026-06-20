Masovne informacije o uporabnikih za najemnika. Glede na userIds vrne prikazne informacije iz User / SSOUser.
Uporabljeno s pripomočkom komentarjev za obogatitev uporabnikov, ki so se pravkar pojavili prek dogodka prisotnosti.
Brez konteksta strani: zasebnost se uveljavlja enotno (zasebni profili so skriti).

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds, ločeni z vejico. |

## Odgovor

Vrača: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## Primer

[inline-code-attrs-start title = 'get_users_info Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je izbirno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco odjemalca API
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | userIds, ločeni z vejico.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]