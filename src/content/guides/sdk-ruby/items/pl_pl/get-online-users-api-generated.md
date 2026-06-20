Aktualnie online widzowie strony: osoby, których sesja websocket jest aktualnie subskrybowana dla tej strony.
Zwraca anonCount + totalCount (subskrybenci w pokoju, włącznie z anonimowymi widzami, których nie enumerujemy).

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak | Identyfikator URL strony (oczyszczany po stronie serwera). |
| afterName | string | query | Nie | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | Nie | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identyfikator URL strony (oczyszczany po stronie serwera).
opts = {
  after_name: 'after_name_example', # String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
  after_user_id: 'after_user_id_example' # String | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]