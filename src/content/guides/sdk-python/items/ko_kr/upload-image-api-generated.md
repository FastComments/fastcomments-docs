이미지 업로드 및 리사이즈

## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | 크기 프리셋: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 디바이스용 크기 생성) |
| urlId | string | query | No | 업로드가 발생하는 페이지 ID, 구성용 |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Example

[inline-code-attrs-start title = 'upload_image 예시'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# configuration.py에서 지원되는 모든 구성 매개변수 목록을 확인하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스로 컨텍스트에 진입
with client.ApiClient(configuration) as api_client:
    # API 클래스 인스턴스 생성
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | 크기 프리셋: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 디바이스용 크기 생성) (optional)
    url_id = 'url_id_example' # str | 업로드가 발생하는 페이지 ID, 구성용 (optional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("PublicApi->upload_image의 응답:\n")
        pprint(api_response)
    except Exception as e:
        print("PublicApi->upload_image 호출 시 예외: %s\n" % e)
[inline-code-end]