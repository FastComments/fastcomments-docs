每个 WordPress 站点都需要自己的 FastComments 账户或租户。WordPress 在每个站点上分别对文章进行编号，因此如果在一个站点的第 5 篇文章和另一个站点的第 5 篇文章都连接到同一个账户，它们将共享同一个评论线程。为此，当账户已经连接了其他站点时，插件的设置会拒绝连接新站点。

[您可以在此查看您已激活的集成。](https://fastcomments.com/auth/my-account/manage-data/integrations)

要运行多个 WordPress 站点，请为每个站点创建一个[租户](https://fastcomments.com/auth/my-account/tenants)，切换到该租户，然后在插件的设置页面连接站点。租户已包含在带有白标的计划中。您也可以为每个站点创建单独的 FastComments 账户，并为每个账户使用不同的电子邮件地址。

如果站点已迁移，例如从暂存环境迁移到生产环境，请在旧站点上停用插件并从已激活的集成中移除旧站点。然后连接新站点。已从旧站点同步的评论会保留在账户中，并会显示在新站点上具有相同 ID 的任何文章中。

您可以在单个 WordPress 安装中使用相同的 FastComments 账户，该安装跨多个域名并在这些域名上拥有相同的内容。