By default, FastComments will show the user's name as they entered it, or how it was passed to us via SSO.

しかし、ユーザー名をマスクしたり、別の方法で表示したりしたい場合があります。たとえば、ユーザー名が Allen Rex の場合、"Allen R." のみを表示したいかもしれません。

This can be done without code in the Widget Customization UI, under the setting called `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commenter Name Format ドロップダウンが開き、Capitalize、Last Initial、All Initials などの選択肢が表示されます'; title='名前形式の変更' app-screenshot-end]

The available formats are:

- Capitalize（例のユーザーを Example User と表示）
- Last Initial（例のユーザーを Example U. と表示）
- All Initials（例のユーザーを E. U. と表示）
- 匿名を表示

The effect of changing this is immediate. User's will still see their full username at the top of the comment area, for themselves, but their comments will show the modified username.

Usernames are masked server-side to protect users.