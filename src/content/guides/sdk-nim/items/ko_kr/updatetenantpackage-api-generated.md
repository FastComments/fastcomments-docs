---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 아니요 |  |

## 응답

반환: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'updateTenantPackage 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateTenantPackage(tenantId = "my-tenant-123", id = "pkg-premium", updateTenantPackageBody = UpdateTenantPackageBody())
if response.isSome:
  let updated = response.get()
  echo "Updated package received:", updated
else:
  echo "Update failed, HTTP status: ", httpResponse.status
[inline-code-end]

---