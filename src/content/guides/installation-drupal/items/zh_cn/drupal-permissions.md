该模块添加了三个 Drupal 权限，您可以在 `People > Permissions` 下按角色分配它们。

- **Administer FastComments** - 可访问位于 `/admin/config/content/fastcomments` 的 FastComments 设置表单。
- **View FastComments** - 查看评论小部件所需的权限。没有此权限，小部件不会渲染。
- **Toggle FastComments** - 允许用户使用字段小部件在每个实体级别启用或禁用评论。

默认情况下，只有具有 `administer site configuration` 权限的用户可以更改 FastComments 设置。如果您希望访客看到该小部件，请将 `View FastComments` 授予匿名和已验证用户。