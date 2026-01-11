Een afbeelding uploaden en formaat wijzigen

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nee | Grootte-voorinstelling: "Default" (1000x1000px) of "CrossPlatform" (maakt formaten voor populaire apparaten) |
| urlId | string | query | Nee | Pagina-id waarvan de upload plaatsvindt, om te configureren |

## Antwoord

Retourneert: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'upload_image Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Grootte-voorinstelling: \"Default\" (1000x1000px) of \"CrossPlatform\" (maakt formaten voor populaire apparaten)
  url_id: 'url_id_example' # String | Pagina-id waarvan de upload plaatsvindt, om te configureren
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]