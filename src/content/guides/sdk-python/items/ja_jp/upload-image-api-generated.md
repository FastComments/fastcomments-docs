Upload and resize an image
============================

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| sizePreset | string | query | いいえ | サイズプリセット: 「Default」(1000x1000px) または「CrossPlatform」(一般的なデバイス向けにサイズを作成) |
| urlId | string | query | いいえ | アップロードが行われるページの ID、設定用 |

## Response

戻り値: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## 例

[inline-code-attrs-start title = 'upload_image 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧を確認できます。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | サイズプリセット: \"Default\" (1000x1000px) または \"CrossPlatform\" (一般的なデバイス向けにサイズを作成) (optional)
    url_id = 'url_id_example' # str | アップロードが行われるページの ID、設定用 (optional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]