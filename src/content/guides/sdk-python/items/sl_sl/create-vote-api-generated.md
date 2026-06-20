## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | query | Da |  |
| direction | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vrača: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Primer

[inline-code-attrs-start title = 'Primer create_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Za seznam vseh podprtih konfiguracijskih parametrov glejte configuration.py.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Spodaj so prikazani primeri za vsako metodo overjanja, uporabite primer, ki
# ustreza vašemu primeru uporabe overjanja.

# Konfigurirajte overjanje z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (neobvezno)
    anon_user_id = 'anon_user_id_example' # str |  (neobvezno)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->create_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_vote: %s\n" % e)
[inline-code-end]