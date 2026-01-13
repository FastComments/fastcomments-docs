[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

デフォルトでは、FastComments はコメントを「Most Relevant」（最も関連性が高い）ソート方向で並べます。

「Most Relevant」ソートは、コメントが投稿された時間と投票数を考慮して並べ替えます。

ユーザーはコメントウィジェットの UI でソート方向を Oldest（古い順）または Newest First（新しい順）に変更できます。

ただし、デフォルトはこれら3つのいずれにも変更できます。例えば、最も古いコメントを先に表示したい場合:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

**defaultSortDirection** の値を "OF" に設定して、方向を "OF" にします。

新しい順（Newest First）のソート方向にするには、次のようにします:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

**defaultSortDirection** に使用できる値は次のとおりです:

- MR: "最新順"
- NF: "新しい順"
- OF: "古い順"

これはコードなしでも行えます。ウィジェットのカスタマイズページの「デフォルトのソート方向」セクションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

注意: 各ページのコメントは各ソート方向ごとに事前計算されているため、すべてのソート方向でパフォーマンスは同じです。