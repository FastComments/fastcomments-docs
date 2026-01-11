---
画像のアップロードとリサイズ

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | サイズプリセット: "Default" (1000x1000px) または "CrossPlatform" (一般的なデバイス向けのサイズを作成します) |
| urlId | string | query | No | アップロードが行われているページのID、設定用 |

## レスポンス

戻り値: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/upload_image_response.rb)

## 例

[inline-code-attrs-start title = 'upload_image の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
file = File.new('/path/to/some/file') # File | 
opts = {
  size_preset: FastCommentsClient::SizePreset::DEFAULT, # SizePreset | サイズプリセット: \"Default\" (1000x1000px) または \"CrossPlatform\" (一般的なデバイス向けのサイズを作成します)
  url_id: 'url_id_example' # String | アップロードが行われているページのID、設定用
}

begin
  
  result = api_instance.upload_image(tenant_id, file, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->upload_image: #{e}"
end
[inline-code-end]

---