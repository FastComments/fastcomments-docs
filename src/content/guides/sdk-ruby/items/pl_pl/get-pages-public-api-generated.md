---
Zwraca listę stron dla najemcy. Używane przez desktopowego klienta FChat do wypełnienia jego listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w ostatecznej niestandardowej konfiguracji dla każdej strony.
Strony wymagające SSO są filtrowane pod kątem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Niejawny kursor paginacji zwrócony jako `nextCursor` z wcześniejszego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny filtr prefiksowy tytułu ignorujący wielkość liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw), lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli true, zwróć tylko strony mające co najmniej jeden komentarz. |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Niejawny kursor paginacji zwrócony jako `nextCursor` z wcześniejszego żądania. Powiązany z tym samym `sortBy`.
  limit: 56, # Integer | 1..200, domyślnie 50
  q: 'q_example', # String | Opcjonalny filtr prefiksowy tytułu ignorujący wielkość liter.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw), lub `title` (alfabetycznie).
  has_comments: true # Boolean | Jeśli true, zwróć tylko strony mające co najmniej jeden komentarz.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]

---