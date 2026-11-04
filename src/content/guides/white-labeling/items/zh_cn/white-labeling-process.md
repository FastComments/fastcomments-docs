设置白标（White Labeling）的流程如下：

1. 选择计费处理方式。
   1. 使用 FastComments Pro，您支付固定金额，可获得一定数量的白标租户。
   2. 使用 FastComments Flex，您按每个租户及其使用量付费。
   3. 在这两种情况下，您都可以设置每个租户的限制。
      1. 限制可以按租户进行自定义。此外，如果您更新所销售的套餐，也不会影响您已向现有客户提供的定价。
2. 熟悉 FastComments.com 的术语：
   1. `Tenant` 是“客户”。
   2. `TenantUser` 是在 `Tenant` 中拥有一组权限的管理员。
   3. `TenantPackage` 是包含一组限制和定价的套餐，可供 `Tenant` 使用。
3. 通过我们的 [API](/guide-api.html) 或使用 [脚本](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling) 来接入租户。