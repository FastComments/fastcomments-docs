## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## 예제

[inline-code-attrs-start title = 'deletePage 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deletePage(tenantId = "site-tenant-456", id = "news/winter-updates-2025")
if response.isSome:
  let deleted = response.get()
  echo "DeletePageAPIResponse:", deleted
else:
  echo "Delete failed, HTTP response:", httpResponse
[inline-code-end]

---