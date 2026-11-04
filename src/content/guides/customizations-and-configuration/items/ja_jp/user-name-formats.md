デフォルトでは、FastCommentsはユーザーが入力した名前、またはSSO経由で渡された名前をそのまま表示します。

しかし、ユーザー名をマスクしたり、別の方法で表示したい場合があります。例えば、ユーザー名がAllen Rexの場合、「Allen R.」のように表示したいかもしれません。

これは、Widget Customization UIの `Commenter Name Format` という設定で、コードを書かずに行うことができます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Capitalize、Last Initial、All Initialsなどの選択肢が表示されたCommenter Name Formatのドロップダウン'; title='名前の形式を変更する' app-screenshot-end]

利用可能な形式は以下の通りです:

- Capitalize(例のユーザーを Example User と表示)
- Last Initial(Example User を Example U. と表示)
- All Initials(Example User を E. U. と表示)
- "Anonymous" を表示

この設定を変更すると、すぐに反映されます。ユーザー本人にはコメント欄の上部に自分の完全なユーザー名が表示されたままですが、コメントには変更後のユーザー名が表示されます。

ユーザー名はユーザーを保護するためにサーバー側でマスクされます。