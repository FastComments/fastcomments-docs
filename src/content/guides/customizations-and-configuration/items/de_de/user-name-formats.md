By default, FastComments will show the user's name as they entered it, or how it was passed to us via SSO.

However, it may be desirable to mask or show the user's name in a different way. For example, if the user's name is Allen Rex, maybe you want to only show "Allen R.".

This can be done without code in the Widget Customization UI, under the setting called `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commenter Name Format Dropdown geöffnet mit Auswahlmöglichkeiten wie Capitalize, Last Initial und All Initials'; title='Namensformat ändern' app-screenshot-end]

The available formats are:

- Capitalize (zeigt Beispielbenutzer als Example User)
- Last Initial (zeigt Example User als Example U.)
- All Initials (zeigt Example User als E. U.)
- Show "Anonymous"

The effect of changing this is immediate. Benutzer sehen weiterhin ihren vollständigen Benutzernamen oben im Kommentarbereich für sich selbst, aber ihre Kommentare zeigen den modifizierten Benutzernamen.

Usernames are masked server-side to protect users.