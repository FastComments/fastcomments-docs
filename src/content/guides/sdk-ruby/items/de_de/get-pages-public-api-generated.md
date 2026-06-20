Listet Seiten für einen Mandanten auf. Wird vom FChat-Desktop-Client verwendet, um dessen Raumliste zu füllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.
Seiten, die SSO erfordern, werden anhand des Gruppenzugriffs des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nein | Opaker Paginierungs-Cursor, der als `nextCursor` aus einer vorherigen Anfrage zurückgegeben wurde. An dasselbe `sortBy` gebunden. |
| limit | integer | query | Nein | 1..200, Standard 50 |
| q | string | query | Nein | Optionaler, nicht case-sensitiver Titelpräfixfilter. |
| sortBy | string | query | Nein | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst), oder `title` (alphabetisch). |
| hasComments | boolean | query | Nein | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_pages_public Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Opaker Paginierungs-Cursor, der als `nextCursor` aus einer vorherigen Anfrage zurückgegeben wurde. An dasselbe `sortBy` gebunden.
  limit: 56, # Integer | 1..200, Standard 50
  q: 'q_example', # String | Optionaler, nicht case-sensitiver Titelpräfixfilter.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst), oder `title` (alphabetisch).
  has_comments: true # Boolean | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]