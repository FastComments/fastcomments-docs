## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | float64 | 아니오 |  |

## 응답

반환: [`Option[GetHashTags_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags200response.nim)

## 예제

[inline-code-attrs-start title = 'getHashTags 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getHashTags(tenantId = "my-tenant-123", page = 1.0)
if response.isSome:
  let tags = response.get()
  for t in tags:
    echo t
else:
  echo "no hashtags found"
[inline-code-end]

---