FastComments is a localized platform. All of our widgets, emails, and notifications are localized.

Localized means that we show a different language, and formatting, based on the user's location
and preferred language. We determine this based off the information the user's browser provides us.

We can customize the text in the email by going to the `Translations` tab, selecting a `Locale`
and editing the text. Text that is changed from the default is outlined in the UI. You may
switch between locales and save at the end, without losing changes.

Localized text is accessed via the `TEXT` object, for example: `<%= TEXT.INTRO %>`.

### SSO Note

For SSO integrations, if `locale` is not specified, it will be updated every time the user
accesses the comment widget with a different locale. This means their language preference
is automatically updated, and future emails will be sent in that locale.

This can also be set manually by providing `locale` in the SSO payload.