---
虽然 FastComments 支持团队可以帮助进行迁移，但大多数迁移可以轻松完成并监控，无需支持人员的干预。

我们原生支持从以下提供商导入导出文件：

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin, or an XML or CSV export)
- AnyComment (Via WordPress Import/Export)

通过访问[此处](https://fastcomments.com/auth/my-account/manage-data/import)可以上传包含要迁移数据的文件。

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments 导入页面，包含提供商选择和用于导出文件的文件上传字段'; title='导入页面表单' app-screenshot-end]

### 监控导入

FastComments 使用作业处理系统来处理导入和导出。一旦系统接收到您的作业，它将定期在导入或导出界面中报告作业状态。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='导入页面显示正在运行的导入任务以及任务处理系统报告的状态'; title='导入任务状态' app-screenshot-end]

请注意，导入和导出的状态对账户中的所有管理员均可查看。

如果您的作业失败，它不会自动重新启动。需要重新尝试导入。如果任何导入或导出失败，我们的系统管理员会自动收到通知。如果我们发现问题，我们会联系您，看看是否能提供帮助。

### 重新运行导入

在某些迁移过程中，需要多次运行导入。例如，通常会先进行一次测试性的迁移，然后在切换之前使用最新数据再次运行导入。

重新导入相同内容 **不会产生重复**。

### 数据安全与过期

导入文件不会以任何方式通过外部请求访问，且导入完成后，导入文件会立即从我们的系统中删除。

---