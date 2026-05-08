[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はページ上にユーザーの一覧を表示しません。

コメントウィジェットの横に、現在ページを閲覧しているユーザーの一覧を表示できます。リストはユーザーの入退出に合わせてライブで更新され、名前、アバター、オンラインインジケーターを表示します。

レイアウトは3種類あります:

- `1` - 上部: コメントの上に表示される、重なり合うアバターの横並び。
- `2` - 左: ウィジェットの左側に表示される、名前とオンラインドット付きのサイドバー。
- `3` - 右: 同じサイドバーがウィジェットの右側に表示されます。

この機能を有効にするには、**usersListLocation** フラグを設定します:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

デフォルトではリストは現在オンラインのユーザーのみを表示します。過去にページにコメントした（現在閲覧していない）人も含めるには、**usersListIncludeOffline** を true に設定してください:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

過去のコメント投稿者は緑のオンラインドットなしで表示され、誰が現在いるかがわかるようになっています。

プロフィール非公開のユーザーは、匿名アバターと「非公開プロフィール」ラベルで表示され、個人を明らかにせずにカウントが正確に保たれます。

この設定はコードなしでも行えます。ウィジェットのカスタマイズページで「ユーザーリストの表示位置」オプションを確認してください。表示位置が Off（オフ）以外に設定されていると、その下に「過去のコメント投稿者を含める」チェックボックスが表示されます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

ライブユーザーが500人を超える場合、リストは最大30秒遅れることがあります。