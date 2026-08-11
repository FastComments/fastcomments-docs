[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーにコメント、ユーザー名、メールアドレスのみを求めます。

しかし、状況によっては、ユーザーに自分のブログやウェブサイトへのリンクを残してもらいたい場合があります。

**enableCommenterLinks** フラグを true に設定することで、ユーザーのウェブサイト URL を入力する追加フィールドを表示できるようにします:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

その URL が提供されると、ユーザーのアカウントが更新され、過去および将来のすべてのコメントでユーザー名がこの URL にリンクされます。

これはコードを書かずに、ウィジェットカスタマイズページでカスタマイズできます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='コメント投稿者リンクのチェックボックスがオンになっており、コメントフォームにウェブサイト URL フィールドが追加されたウィジェットカスタマイズページ'; title='コメント投稿者リンクの有効化' app-screenshot-end]