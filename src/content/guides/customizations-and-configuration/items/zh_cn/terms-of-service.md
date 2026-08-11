FastComments 允许您在提交评论之前，要求首次评论的用户接受您的服务条款。

启用后：
- **匿名用户** 将在每次评论时看到服务条款复选框
- **已认证用户** 只会在他们的首次评论时或当您更新服务条款时看到复选框

### Configuration

导航至小部件自定义页面并勾选 “Require Terms of Service acceptance” 复选框。启用后，您将看到以下选项：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='服务条款面板，显示 TOS 文本模式选择器和最近更新日期字段'; title='服务条款选项' app-screenshot-end]

- **TOS 文本模式**：默认情况下，复选框显示 "I agree to the Terms of Service and Privacy Policy"，并带有指向两个文档的链接。选择 "Customize text per locale" 以为每种语言提供您自己的文本。
- **TOS 最近更新日期**：当您更新服务条款时，设置此日期。之前在此日期之前接受的用户将需要再次接受。

### How It Works

- TOS 接受时间戳按用户和评论分别存储
- 当用户接受 TOS 时，日期会记录在其用户资料中（按租户）
- 如果您设置的 "Last Updated" 日期晚于用户的接受日期，他们将需要重新接受
- 对于无法追踪的匿名用户，复选框会出现在每次评论提交时

---