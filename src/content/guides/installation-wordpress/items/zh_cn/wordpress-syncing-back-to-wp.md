默认情况下，FastComments 每天会将评论同步回您的 WordPress 站点。这样做纯粹是为了备份目的，以便您继续拥有数据的副本，并为可能依赖于它的插件提供支持。

由于某些站点能够处理大量读取流量，但其数据库部署未必能处理大量写入流量（因此将这部分工作卸载给 FastComments），所以这并不会在每次保存评论时立即发生。

可以通过安装插件来自定义同步回 WordPress 的计划。我们推荐 [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description)。

步骤：

1. 安装 WP Crontrol
2. 转到 `Settings -> Cron Schedules`。
3. 转到 `Cron Events` 选项卡。
4. 搜索 `fastcomments_cron_hook`。
5. 编辑该事件。您可以将该钩子配置为每小时运行、每天运行两次、每天运行（默认）或每周运行一次。

也可以随时手动执行同步回 WordPress：进入 FastComments 插件仪表板并选择 `Manually Sync`。您可以选择将数据同步回您的 WP 安装，或将您的 WP 评论重新上传到 FastComments 服务器。