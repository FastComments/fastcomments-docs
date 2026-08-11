When users comment, and they are not logged in, they will be asked to provide their email.

This will create an "unverified session" for that user, and we will ask them to verify that session via email.

For some sites, or applications, it's desirable not to ask the user for their email when commenting or voting.

Enabling anonymous commenting makes the email input field optional. However, we can disable it completely. First, enable anonymous commenting, and then the option to disable the email input fields will appear.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; alt='在自定义 UI 中启用匿名评论后显示的，完全隐藏电子邮件字段的选项'; title='禁用电子邮件输入' app-screenshot-end]

With this on, the email fields will not show at all in all of our commenting products.

Note that, with this configuration, all comments will be unverified unless the user creates an account and logs into
https://fastcomments.com.

You may want to consider [禁用未验证标签](/guide-customizations-and-configuration.html#disable-unverified-label).