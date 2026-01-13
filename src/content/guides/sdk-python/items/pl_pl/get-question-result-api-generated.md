## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_result200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_question_result'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_result200_response import GetQuestionResult200Response
from client.rest import ApiException
from pprint import pprint

# Ustawienie hosta jest opcjonalne i domyślnie to https://fastcomments.com
# Zobacz configuration.py, aby poznać listę wszystkich wspieranych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Poniżej znajdują się przykłady dla każdej metody uwierzytelniania, użyj przykładu, który
# odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Konfiguruj autoryzację kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_result(tenant_id, id)
        print("The response of DefaultApi->get_question_result:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_result: %s\n" % e)
[inline-code-end]