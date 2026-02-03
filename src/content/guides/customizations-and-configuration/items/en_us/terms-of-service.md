FastComments allows you to require first-time commenters to accept your Terms of Service before submitting a comment.

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Enabling Terms of Service

Navigate to the widget customization page and enable the "Require Terms of Service acceptance" checkbox:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

By default, the checkbox displays "I agree to the Terms of Service and Privacy Policy" with links to both documents. You can customize this text per locale if needed:

1. Select "Customize text per locale"
2. Select the locale from the dropdown and enter your custom text

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

When you update your Terms of Service, set the "Last Updated" date. Users who accepted the TOS before this date will be required to accept again:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- The TOS acceptance timestamp is stored per-user and per-comment
- When a user accepts the TOS, the date is recorded on their user profile (per-tenant)
- If you set a "Last Updated" date that is after the user's acceptance date, they will need to re-accept
- For anonymous users who cannot be tracked, the checkbox appears on every comment submission