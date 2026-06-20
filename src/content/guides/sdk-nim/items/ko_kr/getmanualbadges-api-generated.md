## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| sso | string | 아니오 |  |

## 응답

반환: [`Option[GetTenantManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_manual_badges_response.nim)

## 예제

[inline-code-attrs-start title = 'getManualBadges 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getManualBadges(sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")
if response.isSome:
  let badges = response.get()
  echo "Manual badges received:"
  echo badges
else:
  echo "No manual badges returned."
  echo httpResponse
[inline-code-end]

---