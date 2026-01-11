Ανέβασμα και αλλαγή μεγέθους εικόνας

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| sizePreset | string | query | Όχι | Προκαθορισμένο μέγεθος: "Default" (1000x1000px) ή "CrossPlatform" (δημιουργεί μεγέθη για δημοφιλείς συσκευές) |
| urlId | string | query | Όχι | Αναγνωριστικό σελίδας από όπου γίνεται το ανέβασμα, για διαμόρφωση |

## Απάντηση

Επιστρέφει: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα upload_image'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Προκαθορισμένο μέγεθος: \"Default\" (1000x1000px) ή \"CrossPlatform\" (δημιουργεί μεγέθη για δημοφιλείς συσκευές)
  url_id: 'url_id_example' # String | Αναγνωριστικό σελίδας από όπου γίνεται το ανέβασμα, για διαμόρφωση
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]

---