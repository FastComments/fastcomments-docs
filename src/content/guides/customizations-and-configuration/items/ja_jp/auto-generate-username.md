When users comment or vote, and they are not logged in, they will be asked to provide their email and username.

For some sites, asking a visitor to invent a unique username is a hurdle, particularly on mobile. FastComments can
generate a neutral username for each new visitor and prefill it in the username field, like `BraveOtter4172`.

The visitor can leave it as‑is, or replace it with a name of their choosing.

This can be enabled from the Customization UI, under the setting called `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='ウィジェットカスタマイズ UI の「ユーザー名を自動生成」オプション'; title='ユーザー名を自動生成' app-screenshot-end]

#### 動作の概要

- Each generated name is unique. It is checked against existing accounts and reserved for that visitor's browser session, so two visitors are not offered the same name.
- The name is only generated for visitors who do not have one yet. Logged in users, SSO users, and visitors who have already commented keep their existing name.
- It works with or without [anonymous commenting](/guide-customizations-and-configuration.html#allow-anon). With anonymous commenting off, the visitor still enters their email, but no longer has to think of a username.
- A returning visitor who enters an email they have used before is matched to their existing account, and keeps the name on that account.
- If a [Default Username](/guide-customizations-and-configuration.html#default-username) is also set, the generated name takes precedence.