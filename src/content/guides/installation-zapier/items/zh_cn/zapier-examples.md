## 示例 Zap

只需几分钟即可设置的工作流示例。

**获取新评论的通知。** 当触发 New Comment 时，使用 Slack 的 “Send Channel Message” 或 Discord 的 “Send Channel Message”。将评论者姓名、评论内容和页面 URL 映射到消息中。添加域过滤器，以便每个站点通知不同的频道。

**记录每条评论。** 当触发 New Comment 时，使用 Google Sheets 的 “Create Spreadsheet Row”。将 Deleted Comment 作为第二个 Zap，向表格追加一行，包含评论 ID，这样表格即可充当审计日志。

**在评论获批时给作者发送邮件。** 当触发 Updated Comment 并通过 Zapier 过滤器（Approved 为 true）时，使用 Gmail 的 “Send Email”。由于 Updated Comment 会在每次更改时触发，过滤器确保此 Zap 仅在获批时响应。

**将评论者添加到您的 CRM 或邮件列表。** 当触发 New Comment 时，使用 HubSpot 的 “Create or Update Contact” 或 Mailchimp 的 “Add or Update Subscriber”，并使用评论者的电子邮件。在将任何人添加到营销列表之前，请遵守您的隐私政策和当地法律。

**从表单创建评论。** 当触发 Typeform 或 Google Forms 的 “New Response” 时，使用 FastComments 的 Create Comment，并提供您站点用于推荐的页面 URL ID。将 Approved 保持未选中，以便在评论显示前进行审核。

**将公告发布到 Feed。** 当触发 Zapier 的 RSS “New Item in Feed” 时，使用 Create Feed Post，将条目的标题、内容和链接发布到 Feed。

**为成员提供 SSO 用户身份。** 使用 Memberstack、Memberful 或您自己的 webhook，然后执行 Find SSO User，随后在 “find or create” 模式下执行 Create SSO User。

**升级被举报的评论。** 当触发 Updated Comment 并通过过滤器（标记计数大于零）时，使用 Trello 的 “Create Card” 或 Linear 的 “Create Issue”，并附上评论 ID 以及指向审核页面的链接。

**在页面上线时发布。** 当触发 WordPress 或 Ghost 的 “New Post” 时，使用 Create Page，并提供文章 URL，这样页面将在首条评论出现前即被列出并受限。

**归档已删除的评论。** 当触发 Deleted Comment 时，使用 Airtable 的 “Create Record”，并保存完整的评论内容以满足合规保留要求。