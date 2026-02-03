FastComments allows you to require first-time commenters to accept your Terms of Service before submitting a comment.

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Configuration

Navigate to the widget customization page and enable the "Require Terms of Service acceptance" checkbox. Once enabled, you'll see the following options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: By default, the checkbox displays "I agree to the Terms of Service and Privacy Policy" with links to both documents. Select "Customize text per locale" to provide your own text for each language.
- **TOS Last Updated Date**: When you update your Terms of Service, set this date. Users who accepted before this date will be required to accept again.

### How It Works

- The TOS acceptance timestamp is stored per-user and per-comment
- When a user accepts the TOS, the date is recorded on their user profile (per-tenant)
- If you set a "Last Updated" date that is after the user's acceptance date, they will need to re-accept
- For anonymous users who cannot be tracked, the checkbox appears on every comment submission