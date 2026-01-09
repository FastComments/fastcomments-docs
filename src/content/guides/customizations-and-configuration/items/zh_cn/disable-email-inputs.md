当用户发表评论且未登录时，会要求他们提供电子邮件。

这将为该用户创建一个“未验证会话”，我们会通过电子邮件要求他们验证该会话。

对于某些网站或应用，在评论或投票时不要求用户提供电子邮件可能更合适。

启用匿名评论会使电子邮件输入字段为可选。但是，我们也可以完全禁用它。首先启用
匿名评论，然后将出现禁用电子邮件输入字段的选项。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; title='Disable Email Inputs' app-screenshot-end]

开启此项后，我们所有评论产品中的电子邮件字段将完全不显示。

请注意，在此配置下，除非用户创建账户并登录 https://fastcomments.com，否则所有评论都将处于未验证状态。

您可能想考虑[禁用未验证标签](/guide-customizations-and-configuration.html#disable-unverified-label).