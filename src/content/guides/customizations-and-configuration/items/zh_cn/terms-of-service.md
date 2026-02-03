FastComments 允许您要求首次评论者在提交评论之前接受您的服务条款。

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Configuration

Navigate to the widget customization page and enable the "Require Terms of Service acceptance" checkbox. Once enabled, you'll see the following options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: By default, the checkbox displays "I agree to the Terms of Service and Privacy Policy" with links to both documents. Select "Customize text per locale" to provide your own text for each language.
- **TOS Last Updated Date**: When you update your Terms of Service, set this date. Users who accepted before this date will be required to accept again.

### How It Works

- 服务条款接受的时间戳按用户和按评论分别存储
- 当用户接受服务条款时，日期会记录在他们的用户资料中（按租户）
- 如果您设置的 "Last Updated" 日期晚于用户的接受日期，他们将需要重新接受
- 对于无法被跟踪的匿名用户，该复选框会在每次提交评论时出现