如果需要移动数据，FastComments 提供了一个自助工具，用于在页面和文章之间移动评论。

以下是评论复制页面表单的外观：

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='复制评论表单，包含“来源 URL ID”字段和“目标 URL ID 与 URL”字段'; title='复制评论表单' app-screenshot-end]

### 填写 "From" 字段

要决定从哪里移动评论，我们只需知道源 `URL ID`。

如果在评论小部件配置中未传递 `urlId` 的值，则这将是页面 URL 的“干净”版本。

您可以通过导出评论来查看评论的 `URL ID` 值。

### 填写 "To" 字段

要决定将评论移动到哪里，我们需要知道目标 `URL ID` 和 `URL`。

`URL ID` 将是评论所在的桶。`URL` 字段用于您可以直接从电子邮件和审核工具导航到该评论。

#### WordPress

如果您使用 WordPress，例如，您将在迁移工具的 To/From `URL ID` 字段中输入文章 ID，而不是 URL。