By default, FastComments will show the user's name as they entered it, or how it was passed to us via SSO.

However, you may want to mask or show the user's name in a different way. For example, if the user's name is Allen Rex, you may want to only show "Allen R.".

This can be done without code in the Widget Customization UI, under the setting called `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

The available formats are:

- Capitalize (display example user as Example User)
- Last Initial (display Example User as Example U.)
- All Initials (display Example User as E. U.)
- Show "Anonymous"

The effect of changing this is immediate. Users will still see their full username at the top of the comment area, for themselves, but their comments will show
the modified username.

Usernames are masked server-side to protect users.

---