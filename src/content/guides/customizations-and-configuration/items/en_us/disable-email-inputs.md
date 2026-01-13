When users comment and are not logged in, they will be asked to provide their email.

This will create an "unverified session" for that user, and we will ask them to verify that session via email.

For some sites or applications, it's desirable not to ask users for their email when commenting or voting.

Enabling anonymous commenting makes the email input field optional. However, you can disable it completely. First, enable
anonymous commenting, and then the option to disable the email input fields will appear.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments', '.disable-email-inputs']; selector = '.disable-email-inputs'; title='Disable Email Inputs' app-screenshot-end]

With this on, the email fields will not appear at all in any of our commenting products.

Note that with this configuration, all comments will be unverified unless the user creates an account and logs in to
https://fastcomments.com.

You may want to consider [disabling the unverified label](/guide-customizations-and-configuration.html#disable-unverified-label).