[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はページ上にユーザーリストを表示しません。

ページを現在閲覧しているユーザーのリストを、コメントウィジェットと一緒に表示できます。ユーザーが参加したり離脱したりするとリストはリアルタイムで更新され、名前、アバター、オンラインインジケーターが表示されます。

There are three layout options:

- `1` - 上部: コメントの上に表示される、重なり合うアバターの横一列。
- `2` - 左側: ウィジェットの左側に表示される、名前とオンラインドットのサイドバー。
- `3` - 右側: ウィジェットの右側に表示される同様のサイドバー。

Set the **usersListLocation** flag to enable the feature:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = '右側にユーザーリストを表示'; code-example-end]

デフォルトでは、リストは現在オンラインのユーザーのみを表示します。過去にページにコメントしたが現在は閲覧していないユーザーも含めるには、**usersListIncludeOffline** を true に設定します：

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = '過去のコメント投稿者を含める'; code-example-end]

過去のコメント投稿者は緑色のオンラインドットが表示されないため、現在誰がオンラインかが明確になります。

プライベートプロフィールのユーザーは、汎用アバターと「プライベートプロフィール」ラベルで表示され、身元を明かさずにカウントが正確に保たれます。

コードを書かずに設定することもできます。ウィジェットのカスタマイズページで「Users List Location」オプションを確認してください。場所が Off 以外に設定されていると、下に「Include past commenters」チェックボックスが表示されます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='ユーザーリストの位置が右に設定され、下に過去のコメント投稿者を含めるチェックボックスが表示されています'; title='ユーザーリスト設定'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

過去 500 人のライブユーザーについて、リストは最大で 30 秒遅れています。