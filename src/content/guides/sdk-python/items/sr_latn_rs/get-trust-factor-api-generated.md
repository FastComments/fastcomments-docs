## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| userId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## Primer

[inline-code-attrs-start title = 'get_trust_factor Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za spisak svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (opciono)
    sso = 'sso_example' # str |  (opciono)

    try:
        api_response = api_instance.get_trust_factor(user_id=user_id, sso=sso)
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]