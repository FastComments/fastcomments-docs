## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | float64 | 아니오 |  |

## 응답

반환: [`Option[GetQuestionConfigsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_configs_response.nim)

## 예제

[inline-code-attrs-start title = 'getQuestionConfigs 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfigs(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let configs = response.get()
  echo "Received question configs for tenant my-tenant-123"
  echo configs
else:
  echo "No question configs returned"
[inline-code-end]

---