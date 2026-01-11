---
画像をアップロードしてリサイズする

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | サイズプリセット: "Default" (1000x1000px) または "CrossPlatform" (一般的なデバイス向けのサイズを作成する) |
| urlId | string | query | No | アップロードが行われるページのID、設定用 |

## レスポンス

戻り値: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## 例

[inline-code-attrs-start title = 'upload_image の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスでコンテキストに入る
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成する
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | サイズプリセット: \"Default\" (1000x1000px) または \"CrossPlatform\" (一般的なデバイス向けのサイズを作成する) (オプション)
    url_id = 'url_id_example' # str | アップロードが行われるページのID（設定用） (オプション)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]

---