[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はページ上にユーザー一覧を表示しません。

コメントウィジェットの横に、現在そのページを閲覧しているユーザーの一覧を表示できます。リストはユーザーの入退室に応じてリアルタイムに更新され、名前、アバター、オンラインインジケーターを表示します。

レイアウトは3種類あります：

- `1` - Top: コメントの上に重なったアバターが横一列で表示されます。
- `2` - Left: ウィジェットの左側に名前とオンラインドットを表示するサイドバー。
- `3` - Right: 同様のサイドバーがウィジェットの右側に表示されます。

この機能を有効にするには、**usersListLocation** フラグを設定します：

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

デフォルトではリストは現在オンラインのユーザーのみを表示します。過去にそのページにコメントした（現在閲覧していない）ユーザーも含めるには、**usersListIncludeOffline** を true に設定してください：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

過去のコメント投稿者は緑のオンラインドットなしで表示されるため、現在オンラインのユーザーが誰かが明確になります。

プライベートプロフィールのユーザーは、汎用のアバターと「非公開プロフィール」ラベルで表示され、身元を明かさずに人数が正確に保たれます。

これはコードなしでも設定できます。ウィジェットのカスタマイズページで「Users List Location」オプションを参照してください。場所が Off 以外に設定されていると、その下に「Include past commenters」チェックボックスが表示されます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]