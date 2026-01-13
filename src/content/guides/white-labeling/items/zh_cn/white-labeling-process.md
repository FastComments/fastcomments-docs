设置白标（White Labeling）的流程如下：

1. 选择计费方式。
   1. With FastComments Pro, you pay a fixed amount for up to a certain number of white labeled tenants.
   2. With FastComments Flex, you pay for each tenant and the usage of that tenant.
   3. 在这两种情况下，您都需要为每个租户设置限制。
      1. 限制可以针对每个租户单独定制。此外，如果您更新所销售的套餐，可以在不影响已提供给现有客户的价格的情况下进行。
2. 熟悉 FastComments.com 的术语：
   1. `Tenant` 是 "客户"。
   2. `TenantUser` 是在 `Tenant` 中具有一系列权限的管理员。
   3. `TenantPackage` 是为 `Tenant` 提供的一种具有设定限制和定价的套餐。
3. 通过我们的 [API](/guide-api.html) 集成，或使用 [scripts](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling) 来为租户入驻。

---