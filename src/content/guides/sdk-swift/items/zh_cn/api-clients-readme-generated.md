FastComments SDK 提供三个 API 客户端：

### PublicAPI - 客户端安全方法

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- 不需要 API 密钥
- 可以使用 SSO 令牌进行身份验证
- 对每个用户/设备进行速率限制
- 适用于面向终端用户的应用

**示例用例**：在你的 iOS 应用中获取和创建评论

### DefaultAPI - 服务器端方法

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- 需要你的 FastComments API 密钥
- 仅应从服务器端代码调用
- 提供对你的 FastComments 数据的完全访问
- 对每个租户进行速率限制

**示例用例**：管理操作、批量数据导出、用户管理

### ModerationAPI - 版主仪表板方法

The `ModerationAPI` contains methods that power the moderator dashboard. These methods cover:
- **Comment moderation** - 列出、计数、搜索、检索日志和导出评论
- **Moderation actions** - 删除/恢复评论、标记、设置审核/垃圾/批准状态、管理投票，以及重新打开/关闭主题
- **Bans** - 对用户进行评论封禁、撤销封禁、获取封禁前摘要、检查封禁状态和偏好，以及读取被封禁用户计数
- **Badges & trust** - 授予/移除徽章、列出手动徽章、获取/设置用户的信任因子，以及读取用户的内部资料

Every `ModerationAPI` method accepts an `sso` parameter so moderators can be authenticated via SSO.

**示例用例**：为你的社区版主构建审核体验

**重要**：切勿在客户端代码中暴露你的 API 密钥。API 密钥应仅在服务器端使用。