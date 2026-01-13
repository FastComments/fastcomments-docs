---
管理者向けには、コメントモデレーションページの上部に "モデレーターを追加" ボタンがあります。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=3&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.moderation-settings-options'; title='Comment Moderation Setting Buttons' app-screenshot-end]

もし既にモデレーターがいる場合、このボタンは "モデレーターを編集" と表示されます。

それでは "モデレーターを追加" ページを見てみましょう。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='The Add a Moderator Page' app-screenshot-end]

モデレーターを追加するには、名前とメールアドレスだけが必要です。

そのメールアドレスが既存の FastComments アカウントに紐づいている場合、そのユーザーはあなたのアカウントのモデレーターとして参加するようメールで招待されます。

指定されたメールアドレスが既存の FastComments アカウントに紐づいていない場合、新しいアカウントが作成されます。

モデレーターには招待リンクが送信され、これにより自動的にログインできます。将来的にログインしたい場合は、単に次のようにしてください。
<a href="https://fastcomments.com/auth/login" target="_blank">ログインページ</a> にアクセスして、先に入力した名前／メールアドレスを入力します。すると
ログイン用のリンクが送られます。

ログアウトしない限り、ログイン状態は30日間保持されます。

---