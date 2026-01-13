이미지 업로드 및 크기 조정

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| sizePreset | string | query | 아니요 | 크기 프리셋: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 기기에 대한 크기를 생성) |
| urlId | string | query | 아니요 | 업로드가 발생하는 페이지 ID, 구성용 |

## 응답

반환: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## 예제

[inline-code-attrs-start title = 'upload_image 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 지정은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | 크기 프리셋: \"Default\" (1000x1000px) 또는 \"CrossPlatform\" (인기 기기에 대한 크기를 생성) (선택 사항)
    url_id = 'url_id_example' # str | 업로드가 발생하는 페이지 ID, 구성용 (선택 사항)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]

---