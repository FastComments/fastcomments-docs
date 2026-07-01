Wyświetla listę stron dla najemcy. Używane przez klienta desktopowego FChat do wypełniania jego listy pokoi.  
Wymaga, aby `enableFChat` było ustawione na **true** w rozwiązanej niestandardowej konfiguracji dla każdej strony.  
Strony, które wymagają SSO, są filtrowane w oparciu o dostęp grupowy użytkownika żądającego.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Nietransparentny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny filtr prefiksu tytułu nie uwzględniający wielkości liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw) lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli **true**, zwróć tylko strony z co najmniej jednym komentarzem. |

## Odpowiedź

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Przykład

[inline-code-attrs-start title = 'get_pages_public Przykład'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definiowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Nietransparentny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. (optional)
    limit = 56 # int | 1..200, domyślnie 50 (optional)
    q = 'q_example' # str | Opcjonalny filtr prefiksu tytułu nie uwzględniający wielkości liter. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw) lub `title` (alfabetycznie). (optional)
    has_comments = True # bool | Jeśli **true**, zwróć tylko strony z co najmniej jednym komentarzem. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]