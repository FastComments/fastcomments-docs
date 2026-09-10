## 操作和搜索

操作在 FastComments 中创建数据；搜索则查找数据，以便后续步骤使用。每个操作都会调用 FastComments REST API，并消耗与您自己的代码调用相同的 API 积分：每次调用消耗 1 积分，除非另有说明。

## 创建评论

在页面上发布评论。

| Field | Required | Notes |
|-------|----------|-------|
| Page URL ID | Yes | 评论小部件在页面上使用的 URL ID。评论会按该 ID 分组。 |
| Page URL | Yes | 完整的页面 URL，用于通知邮件。 |
| Comment | Yes | FastComments markdown 格式的评论正文。 |
| Commenter Name | Yes | 名称在每个电子邮件下唯一，因此使用不同电子邮件重复使用同一名称会失败。 |
| Commenter Email | No | 当该电子邮件尚不存在时，会为其创建用户。 |
| User ID | No | 已有的 SSO 用户 ID。优先于名称和电子邮件。 |
| Parent Comment ID | No | 设置后发布回复。 |
| Approved, Verified | No | 默认均为 true。未批准的评论在被审核前保持隐藏。 |
| Posted At | No | 默认值为当前时间。 |
| Avatar URL, Page Title, Locale | No | Locale 默认值为 `en_us`。 |
| Show Live In Widget | No | 实时推送评论给观看者。消耗 2 积分而非 1 积分。 |
| Run Spam Check, Send Emails | No | 默认关闭。 |

## 创建页面

在页面上出现任何评论之前创建页面记录，以便对其进行列出和限制。需要提供 URL ID、标题、URL，并可选地提供允许查看的 SSO 组 ID。

## 创建 SSO 用户

创建单点登录用户。需要提供您自己的用户 ID、用户名和电子邮件，以及可选的显示名称、显示标签、头像、网站、组 ID、以及通知和隐私标志。无法通过 Zapier 授予管理角色。

## 创建 Feed 帖子

从 HTML 内容在 FastComments feed 中创建帖子，可选提供标题、作者、标签以及一个链接预览。

## 创建标签

创建评论者可使用的标签，可选提供其链接的 URL。

## 标记评论

将评论标记为需要审核。提供执行标记的用户 ID，或留空以使用 Zapier 集成的身份标记。

## 搜索

| 搜索 | 输入 | 返回 |
|--------|-------|---------|
| Find Comment | Comment ID | 评论，或无结果。 |
| Find SSO User | Email | SSO 用户，或无结果。 |
| Find Page | URL ID | 页面，或无结果。 |

搜索未找到结果时不会导致 Zap 失败。可在 Zapier 的 "find or create" 模式下将搜索与创建结合，在页面或用户缺失时自动创建它们。