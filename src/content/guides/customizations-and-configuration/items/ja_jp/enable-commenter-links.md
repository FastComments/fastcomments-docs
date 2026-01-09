[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーに対してコメント、ユーザー名、メールアドレスのみを求めます。

しかし、場合によってはユーザーに自分のブログやウェブサイトへのリンクを残してもらいたいことがあります。

ユーザーのウェブサイトURLを入力するための追加入力フィールドを表示するには、**enableCommenterLinks** フラグを true に設定します:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

そのURLが提供されると、ユーザーのアカウントが更新され、過去および将来のすべてのコメントに表示されるユーザー名がこのURLにリンクされます。

これはコードなしで、ウィジェットのカスタマイズページで設定できます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---