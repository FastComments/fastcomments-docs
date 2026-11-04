[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はページ上にユーザーのリストを表示しません。

コメントウィジェットと並べて、現在ページを閲覧しているユーザーのリストを表示できます。このリストは、ユーザーが参加・退出するたびにリアルタイムで更新され、名前、アバター、オンライン状態のインジケーターが表示されます。

レイアウトには 3 つのオプションがあります:

- `1` - 上: コメントの上に、重なり合うアバターを横一列に表示します。
- `2` - 左: ウィジェットの左側に、名前とオンラインドットを含むサイドバーを表示します。
- `3` - 右: 同じサイドバーをウィジェットの右側に表示します。

**usersListLocation** フラグを設定して、この機能を有効にします:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'ユーザーリストを右側に表示'; code-example-end]

デフォルトでは、リストには現在オンラインのユーザーのみが表示されます。過去にページにコメントしたことがあるユーザー(現在は閲覧していないユーザー)も含めるには、**usersListIncludeOffline** を true に設定します:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = '過去のコメント投稿者を含める'; code-example-end]

過去のコメント投稿者には緑のオンラインドットが表示されないため、今この瞬間に誰がいるのかが明確になります。

プライベートプロフィールのユーザーは、汎用のアバターと「Private Profile」ラベル付きで表示されるため、身元を明かすことなくカウントは正確に保たれます。

コードを使わずに設定することもできます。ウィジェットのカスタマイズページで「Users List Location」オプションを確認してください。場所が Off 以外に設定されると、その下に「過去のコメント投稿者を含める」チェックボックスが表示されます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Users List Location が Right に設定され、その下に過去のコメント投稿者を含めるチェックボックスが表示されている'; title='ユーザーリスト設定'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

同時接続ユーザーが 500 名を超える場合、リストは最大 30 秒遅れて更新されます。