## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| sso | string | 否 |  |

## 回應

回傳: [`Option[GetBannedUsersCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_count_response.nim)

## 範例

[inline-code-attrs-start title = 'getCounts 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCounts(sso = "sso_my-tenant-123_token_AbCdEf123456")
if response.isSome:
  let counts = response.get()
  echo counts
else:
  echo "Request failed with status:", httpResponse.status
[inline-code-end]

---