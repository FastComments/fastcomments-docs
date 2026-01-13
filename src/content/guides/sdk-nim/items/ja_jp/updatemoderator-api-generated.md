## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updateModeratorBody | UpdateModeratorBody | いいえ |  |

## レスポンス

返却値: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'updateModerator の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateModeratorBody(
  name: "Alicia Gomez",
  email: "alicia.gomez@dailynews.com",
  active: true,
  roles: @["moderator"]
)

let (response, httpResponse) = client.updateModerator(tenantId = "my-tenant-123", id = "moderator-789", updateModeratorBody = updateBody)

if response.isSome:
  let updated = response.get()
  echo "Moderator updated:", updated
[inline-code-end]

---