Poprzedni komentujący na stronie, którzy NIE są obecnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyświetlić sekcję „Członkowie”.
Paginacja kursora po commenterName: serwer przeszukuje częściowy indeks {tenantId, urlId, commenterName}
indeks od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| urlId | string | zapytanie | Tak | Identyfikator URL strony (oczyszczany po stronie serwera). |
| afterName | string | zapytanie | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | zapytanie | Nie | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby wpisy o tej samej nazwie nie zostały pominięte. |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py dla listy wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identyfikator URL strony (oczyszczany po stronie serwera).
    after_name = 'after_name_example' # str | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. (opcjonalne)
    after_user_id = 'after_user_id_example' # str | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby wpisy o tej samej nazwie nie zostały pominięte. (opcjonalne)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]