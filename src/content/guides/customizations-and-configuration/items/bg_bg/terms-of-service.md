FastComments ви позволява да изисквате от потребителите, които коментират за първи път, да приемат вашите Условия за ползване преди да изпратят коментар.

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Configuration

Навигирайте до страницата за персонализиране на уиджета и включете отметката "Require Terms of Service acceptance". След като е включена, ще видите следните опции:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: По подразбиране отметката показва "I agree to the Terms of Service and Privacy Policy" с връзки към двата документа. Изберете "Customize text per locale", за да предоставите собствен текст за всеки език.
- **TOS Last Updated Date**: Когато актуализирате вашите Условия за ползване, задайте тази дата. Потребителите, които са приели преди тази дата, ще трябва да приемат отново.

### How It Works

- The TOS acceptance timestamp is stored per-user and per-comment
- When a user accepts the TOS, the date is recorded on their user profile (per-tenant)
- If you set a "Last Updated" date that is after the user's acceptance date, they will need to re-accept
- For anonymous users who cannot be tracked, the checkbox appears on every comment submission