要使用 API 创建白标租户，我们必须执行以下操作：

1. 调用 [Tenants API](/guide-api.html#tenant-structure) 来创建租户。
2. 调用 [TenantPackages API](/guide-api.html#tenant-package-structure) 为租户创建一个或多个套餐。
3. 调用 [Tenants API](/guide-api.html#tenant-patch) 来定义哪个套餐在租户上处于激活状态。
4. 调用 [TenantUsers API](/guide-api.html#tenant-user-structure) 向租户添加管理员用户。
5. 调用 [Moderators API](/guide-api.html#moderator-structure) 向租户添加并邀请版主。
6. 可选地， [Setup SSO](/guide-customizations-and-configuration.html#sso)。

以上所有操作均可在试用期内完成。

---