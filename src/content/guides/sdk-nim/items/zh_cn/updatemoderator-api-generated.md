## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateModeratorBody | UpdateModeratorBody | 否 |  |

## 响应

返回: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 示例

[inline-code-attrs-start title = 'updateModerator 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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