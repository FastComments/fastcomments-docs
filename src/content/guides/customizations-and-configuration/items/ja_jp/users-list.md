[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はページ上にユーザー一覧を表示しません。

コメントウィジェットの横に、そのページを現在閲覧しているユーザーの一覧を表示できます。リストはユーザーの入退室に合わせてリアルタイムに更新され、名前、アバター、およびオンラインの表示が表示されます。

レイアウトは以下の3種類があります:

- `1` - 上部: コメントの上に重なり合ったアバターを横並びで表示します。
- `2` - 左: ウィジェットの左側に、名前とオンラインドットを表示するサイドバーです。
- `3` - 右: 上記と同様のサイドバーをウィジェットの右側に表示します。

この機能を有効にするには、**usersListLocation** フラグを設定します:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'ユーザー一覧を右側に表示'; code-example-end]

デフォルトではリストは現在オンラインのユーザーのみを表示します。過去にそのページにコメントしたことがある（現在は閲覧していない）ユーザーも含めるには、**usersListIncludeOffline** を true に設定します:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = '過去のコメント投稿者を含める'; code-example-end]

過去のコメント投稿者は緑色のオンラインドットなしで表示されるため、現在オンラインのユーザーが誰かがわかりやすくなります。

プライベートプロフィールのユーザーは、一般的なアバターと "非公開プロフィール" ラベルで表示され、身元を明かさずにカウントは正確に保たれます。

これはコードなしでも設定できます。ウィジェットのカスタマイズページで、「ユーザー一覧の表示位置」オプションを参照してください:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='ユーザー一覧の表示位置' app-screenshot-end]

ロケーションが Off 以外に設定されている場合、その下に「過去のコメント投稿者を含める」チェックボックスが表示されます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='過去のコメント投稿者を含める' app-screenshot-end]