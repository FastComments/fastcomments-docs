虽然 FastComments 支持可以帮助迁移，但大多数迁移可以在无需任何干预的情况下轻松执行和监控
支持人员的介入。

We natively support importing exports from the following providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (通过插件)

By navigating [here](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### 监控导入

FastComments 使用一个作业处理系统来处理导入和导出。一旦系统接手您的作业，它会
定期在导入或导出 UI 中报告作业状态。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

请注意，导入和导出的状态可由帐户中的所有管理员查看。

如果您的作业失败，它不会自动重启。必须再次尝试导入。如果任何导入或导出失败，
我们的系统管理员会自动收到通知。如果我们发现问题，我们会与您联系，看看是否可以提供帮助。

### 重新运行导入

在某些迁移过程中，可能需要多次运行导入。例如，通常会先进行一次测试性
迁移，然后在切换之前使用最新数据再次运行导入。

重新导入相同内容 **不会创建重复项**。

### 数据安全和过期

导入文件不会以任何方式对外部请求开放，并且导入完成后导入文件会从我们的系统中删除。

---