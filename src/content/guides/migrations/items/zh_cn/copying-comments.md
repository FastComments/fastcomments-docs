如果需要移动数据，FastComments 提供了一个自助工具，用于在页面和文章之间移动评论
。

下面是评论复制页面表单的外观：

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### 填写 "From" 字段

要决定从哪里移动评论，我们只需要知道源 `URL ID`。

如果您在评论小部件配置中没有传递 `urlId` 的值，那么这将是页面 URL 的“干净”版本。

您可以通过导出查看评论的 `URL ID` 值。

### 填写 "To" 字段

要决定将评论移至何处，我们需要知道目标 `URL ID` 和 `URL`。

`URL ID` 将是评论所属的存储桶。 `URL` 字段用于使您能够直接
从电子邮件和管理工具导航到该评论。

#### WordPress

如果您使用 WordPress，例如，您会在迁移工具的 To/From `URL ID` 字段中输入文章 ID，
而不是 URL。