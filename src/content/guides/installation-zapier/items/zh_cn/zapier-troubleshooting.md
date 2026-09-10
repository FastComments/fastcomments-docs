## Troubleshooting

**"You do not have permission" 在连接时出现。** 已登录的用户不是该账户的 API 管理员。请让账户所有者在 Users 页面授予 API 权限，或以所有者身份连接。

**连接标记了错误的站点。** 同意页面会连接您当时登录的账户。 在 Zapier 中断开连接，在 FastComments 仪表板切换账户，然后重新连接。

**事件停止到达。** 在仪表板的 Webhooks 页面检查。 如果订阅的端点连续六天失败，系统会自动禁用并显示原因。 在那里重新启用，或关闭再打开 Zap。 如果订阅完全消失，说明有人删除了它；关闭再打开 Zap 会重新创建。

**Zapier 提示需要重新连接账户。** 连接可能已在 Connected Apps 页面被撤销，批准连接的用户失去了 API 权限，或账户被删除。 在 Zapier 中重新连接。

**操作因 "does not have write access" 失败。** 连接时仅批准了只读权限。 重新连接并批准两项权限。

**速率限制和配额。** 操作和搜索会消耗您计划中的 API 配额，并受与 REST API 相同的速率限制。 触发器不消耗配额。 当 Zap 遇到限制时，Zapier 会在 FastComments 报告的延迟后重试。

**Domain 下拉列表为空。** 在 FastComments 仪表板的 Domains 页面配置后，域名会出现。 将该字段留空可接收所有域名的事件。