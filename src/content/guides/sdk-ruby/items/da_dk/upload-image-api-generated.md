Upload og ændre størrelse på et billede

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nej | Størrelsesforindstilling: "Default" (1000x1000px) eller "CrossPlatform" (opretter størrelser til populære enheder) |
| urlId | string | query | Nej | Side-id som uploaden sker fra, til konfiguration |

## Svar

Returnerer: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## Eksempel

[inline-code-attrs-start title = 'upload_image Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | Størrelsesforindstilling: \"Default\" (1000x1000px) eller \"CrossPlatform\" (opretter størrelser til populære enheder)
  url_id: 'url_id_example' # String | Side-id som uploaden sker fra, til konfiguration
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]