FastComments 允许您要求首次评论者在提交评论前接受您的服务条款。

启用后：
- **匿名用户**在每次评论时都会看到服务条款复选框
- **已认证的用户**仅在他们的第一次评论时，或当您更新服务条款时，才会看到该复选框

### 启用服务条款

导航到小部件自定义页面并启用 "Require Terms of Service acceptance" 复选框：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### 自定义服务条款文本

默认情况下，复选框显示 "I agree to the Terms of Service and Privacy Policy"，并包含指向两个文档的链接。您可以按语言/区域自定义此文本（如有需要）：

1. 选择 "Customize text per locale"
2. 从下拉菜单选择语言/区域，然后输入自定义文本

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### 更新您的服务条款

当您更新服务条款时，设置 "Last Updated" 日期。 在该日期之前接受过服务条款的用户将被要求再次接受：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### 工作原理

- 服务条款接受时间戳按用户和评论分别存储
- 当用户接受服务条款时，日期会记录在其用户资料上（按租户）
- 如果您设置的 "Last Updated" 日期晚于用户的接受日期，他们将需要重新接受
- 对于无法被追踪的匿名用户，复选框会出现在每次提交评论时