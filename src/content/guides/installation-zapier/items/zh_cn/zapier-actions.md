## 操作和搜索

操作在 FastComments 中创建数据；搜索则查找数据，以便后续步骤使用。每个操作都会调用
FastComments REST API，并消耗与您自己的代码调用相同的 API 积分：每次调用消耗 1
积分，除非另有说明。

## 创建评论

在页面上发布一条评论。

| 字段 | 必填 | 备注 |
|-------|----------|-------|
| Page URL ID | 是 | 评论小部件在页面上使用的 URL ID。评论会按此分组。 |
| Page URL | 是 | 完整的页面 URL，用于通知邮件。 |
| Comment | 是 | FastComments markdown 格式的评论正文。 |
| Commenter Name | 是 | 名称在每个邮箱下唯一，使用不同邮箱重复使用同一名称会失败。 |
| Commenter Email | 否 | 当邮箱尚不存在时会为其创建用户。 |
| User ID | 否 | 已存在的 SSO 用户 ID。优先于名称和邮箱。 |
| Parent Comment ID | 否 | 设置后发布为回复。 |
| Approved, Verified | 否 | 默认均为 true。未批准的评论会保持隐藏，直到被审核。 |
| Posted At | 否 | 默认使用当前时间。 |
| Avatar URL, Page Title, Locale | 否 | Locale 默认值为 `en_us`。 |
| Show Live In Widget | 否 | 实时推送评论给观看者。费用为 2 积分，而非 1 积分。 |
| Run Spam Check, Send Emails | 否 | 默认关闭。 |

## 创建或更新页面

在任何评论出现之前创建页面记录，以便能够列出并限制访问。需要提供 URL ID、标题、URL，及可选的允许查看的 SSO 组 ID。如果已存在相同 URL ID 的页面，则使用提供的字段进行更新，这样 Zap 可以对同一页面重复运行。

## 创建或更新 SSO 用户

创建单点登录用户。需要提供您自己的用户 ID、用户名和邮箱，外加可选的显示名称、显示标签、头像、网站、组 ID，以及通知和隐私标志。如果已存在相同 ID 的用户，则改为更新。Zapier 无法授予管理角色。

## 创建 Feed 帖子

从 HTML 内容在 FastComments feed 中创建帖子。需要提供作者用户 ID（FastComments 或 SSO 用户 ID）；标题、标签和一个链接预览为可选项。

## 创建或更新标签

创建评论者可使用的标签，可选地提供其链接的 URL。如果标签已存在，则改为更新。

## 标记评论

将评论标记为需要审核。需要提供执行标记的用户 ID；Create Comment 返回的作者 ID 可使用。

## 搜索

| 搜索 | 输入 | 返回 |
|--------|-------|---------|
| Find Comment | Comment ID | 评论对象，若未找到则返回空。 |
| Find SSO User | Email | SSO 用户对象，若未找到则返回空。 |
| Find Page | URL ID | 页面对象，若未找到则返回空。 |

未找到结果的搜索不会导致 Zap 失败。Find SSO User 和 Find Page 提供 Zapier 的 “如果不存在则创建” 选项，当未找到时会运行相应的创建操作。