Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον επιτραπέζιο πελάτη FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτεί `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση (custom config) για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του ζητούντος χρήστη.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου ανεξάρτητο από πεζά/κεφαλαία. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτες οι νεότερες), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Αν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`.
  limit: 56, # Integer | 1..200, προεπιλογή 50
  q: 'q_example', # String | Προαιρετικό φίλτρο προθέματος τίτλου ανεξάρτητο από πεζά/κεφαλαία.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτες οι νεότερες), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά).
  has_comments: true # Boolean | Αν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]