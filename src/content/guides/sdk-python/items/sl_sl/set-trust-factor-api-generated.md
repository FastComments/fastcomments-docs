## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| trustFactor | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## Primer

[inline-code-attrs-start title = 'set_trust_factor Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import SetTrustFactorOptions
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst s primerek API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite primerek API razreda
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (neobvezno)
    trust_factor = 'trust_factor_example' # str |  (neobvezno)
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.set_trust_factor(tenant_id, SetTrustFactorOptions(user_id=user_id, trust_factor=trust_factor, sso=sso))
        print("Odgovor ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Izjema pri klicanju ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]