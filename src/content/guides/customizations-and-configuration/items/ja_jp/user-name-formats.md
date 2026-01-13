デフォルトでは、FastComments はユーザーが入力した名前、または SSO を介して渡された名前をそのまま表示します。

しかし、ユーザー名をマスクしたり別の形式で表示したい場合もあります。例えば、ユーザー名が Allen Rex の場合、もしかしたら
"Allen R." のみを表示したいかもしれません。

This can be done without code in the Widget Customization UI, under the setting called `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

The available formats are:

- Capitalize (display example user as Example User)
- Last Initial (display Example User as Example U.)
- All Initials (display Example User as E. U.)
- Show "Anonymous"

The effect of changing this is immediate. User's will still see their full username at the top of the comment area, for themselves, but their comments will show
the modified username.

Usernames are masked server-side to protect users.

---