When users comment, and they are not logged in, they will be asked to provide their email.

This will create an "unverified session" for that user, and we will ask them to verify that session via email.

For some sites, or applications, it's desirable not to ask the user for their email when commenting or voting.

Enabling anonymous commenting makes the email input field optional. However, we can disable it completely. First, enable
anonymous commenting, and then the option to disable the email input fields will appear.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; alt='匿名コメントが有効になった後に表示される、メールフィールドを完全に非表示にするオプション'; title='メール入力を無効化' app-screenshot-end]

With this on, the email fields will not show at all in all of our commenting products.

Note that, with this configuration, all comments will be unverified unless the user creates an account and logs into
https://fastcomments.com.

You may want to consider [未確認ラベルを無効化](/guide-customizations-and-configuration.html#disable-unverified-label).