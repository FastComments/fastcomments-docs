## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateAPISSOUserData | UpdateAPISSOUserData | No |  |
| updateComments | bool | No |  |

## 응답

반환: [`Option[PatchSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_sso_user_api_response.nim)

## 예시

[inline-code-attrs-start title = 'patchSSOUser 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPISSOUserData()
let (responseOpt, httpResponse) = client.patchSSOUser(
  tenantId = "my-tenant-123",
  id = "user-789",
  updateAPISSOUserData = updateData,
  updateComments = true)
if responseOpt.isSome:
  let response = responseOpt.get()
  echo response
echo httpResponse.statusCode
[inline-code-end]