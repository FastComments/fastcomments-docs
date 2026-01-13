## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_config200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_config200_response import GetQuestionConfig200Response
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką zabezpieczeń serwera API.
# Poniżej podano przykłady dla każdej metody uwierzytelniania; użyj przykładu, który
# odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Skonfiguruj uwierzytelnianie za pomocą klucza API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to potrzebne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź do kontekstu z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_config(tenant_id, id)
        print("The response of DefaultApi->get_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_config: %s\n" % e)
[inline-code-end]

---