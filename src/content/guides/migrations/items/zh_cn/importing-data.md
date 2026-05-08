虽然 FastComments 支持团队可以协助迁移，但大多数迁移可以轻松执行并在无需支持人员介入的情况下进行监控。

我们原生支持从以下提供者导入导出的数据：

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress（通过插件）
- AnyComment（通过 WordPress 导入/导出）

By navigating [here](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='导入页面表单' app-screenshot-end]

### 监控导入

FastComments 使用作业处理系统来处理导入和导出。一旦系统开始处理您的作业，它会定期在导入或导出界面中报告该作业的状态。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='导入任务状态' app-screenshot-end]

请注意，导入和导出的状态可由帐户中的所有管理员查看。

如果您的作业失败，它不会自动重新启动。必须再次尝试导入。如果任何导入或导出失败，我们的系统管理员会自动收到通知。如果我们发现问题，我们会联系您，看看是否可以提供帮助。

### 重新运行导入

在某些迁移过程中，有必要多次运行导入。例如，通常会先进行一次测试性迁移，然后在切换之前使用最新数据再次运行导入。

重新导入相同内容**不会创建重复项**。

### 数据安全与过期

导入文件不会以任何方式通过外部请求被访问，且导入完成后导入文件会立即从我们的系统中删除。

---