Wyświetla listę stron dla najemcy. Używane przez klienta desktopowego FChat do wypełniania jego listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązanej konfiguracji niestandardowej dla każdej strony.
Strony wymagające SSO są filtrowane w oparciu o dostęp grupowy użytkownika wysyłającego żądanie.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Niejawny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny filtr prefiksu tytułu niewrażliwy na wielkość liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najpierw najnowsze), `commentCount` (najpierw strony z największą liczbą komentarzy), lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem. |

## Response

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Niejawny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. (opcjonalne)
    limit = 56 # int | 1..200, domyślnie 50 (opcjonalne)
    q = 'q_example' # str | Opcjonalny filtr prefiksu tytułu niewrażliwy na wielkość liter. (opcjonalne)
    sort_by = client.PagesSortBy() # PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najpierw najnowsze), `commentCount` (najpierw strony z największą liczbą komentarzy), lub `title` (alfabetycznie). (opcjonalne)
    has_comments = True # bool | Jeśli true, zwracaj tylko strony z co najmniej jednym komentarzem. (opcjonalne)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]