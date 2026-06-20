Zbiorcze informacje o użytkownikach dla najemcy. Dla podanych userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widżet komentarzy do wzbogacania informacji o użytkownikach, którzy właśnie pojawili się poprzez zdarzenie obecności.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (profile prywatne są maskowane).

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| ids | string | query | Tak | Identyfikatory userIds rozdzielone przecinkami. |

## Odpowiedź

Zwraca: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | Identyfikatory userIds rozdzielone przecinkami.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---