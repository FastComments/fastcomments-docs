SDK 通过 `CommentWidgetConfig` 类提供了许多配置选项：

| 选项 | 描述 |
|--------|-------------|
| `tenantId` | 您的 FastComments 账户 ID |
| `urlId` | 代表当前页面的 ID |
| `sso` | 用于身份验证的 SSO 令牌 |
| `allowAnon` | 允许匿名评论 |
| `voteStyle` | UpDown 或 Heart 投票样式 |
| `hideAvatars` | 隐藏用户头像 |
| `hasDarkBackground` | 指示深色模式 |
| `customCSS` | 自定义 CSS 样式 |
| `enableInfiniteScrolling` | 启用无限滚动分页 |
| `readonly` | 禁用评论但显示已有评论 |
| `disableVoting` | 禁用投票功能 |
| `disableLiveCommenting` | 禁用实时更新 |
| `pollCreation` | 谁可以在评论中附加投票：`NUMBER_0` 禁用，`NUMBER_1` 管理员和版主，`NUMBER_2` 所有人 |
| `pollVoting` | 谁可以对投票进行投票：`NUMBER_0` 已登录用户，`NUMBER_1` 任何人 |