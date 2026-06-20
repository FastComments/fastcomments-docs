## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'deleteQuestionConfig 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "")

if response.isSome:
  let deleted = response.get()
  echo "Question config deleted for tenant: ", "my-tenant-123"
else:
  echo "Failed to delete question config"
[inline-code-end]

---