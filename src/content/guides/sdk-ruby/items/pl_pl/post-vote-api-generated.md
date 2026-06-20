## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | ścieżka | Tak |  |
| direction | string | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`VoteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład post_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # String | 
opts = {
  direction: 'direction_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_vote(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_vote: #{e}"
end
[inline-code-end]