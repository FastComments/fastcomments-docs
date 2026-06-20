Επιστρέφει λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον desktop client FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτείται το `enableFChat` να είναι true στην επιλυμένη custom config για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται σύμφωνα με την πρόσβαση ομάδων του αιτούμενου χρήστη.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| cursor | string | query | Όχι | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | Όχι | 1..200, προεπιλογή 50 |
| q | string | query | Όχι | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων. |
| sortBy | string | query | Όχι | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτες οι πιο πρόσφατες), `commentCount` (πρώτες αυτές με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | Όχι | Εάν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

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
  q: 'q_example', # String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτες οι πιο πρόσφατες), `commentCount` (πρώτες αυτές με τα περισσότερα σχόλια), ή `title` (αλφαβητικά).
  has_comments: true # Boolean | Εάν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]

---