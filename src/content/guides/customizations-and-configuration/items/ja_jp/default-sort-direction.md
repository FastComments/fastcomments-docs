[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

デフォルトでは、FastComments はコメントを「最も関連性が高い」ソート方向で並べ替えます。

「最も関連性が高い」ソートは、コメントが投稿された時間と投票数を考慮して並べ替えます。

ユーザーはコメントウィジェットの UI で、ソート方向を「最も古い」または「最新」へ変更できます。

ただし、デフォルトを3つのいずれかに変更できます。たとえば、最も古いコメントを最初に表示したい場合は次のようにします：

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'デフォルトのソートを最も古い順に変更する'; code-example-end]

**defaultSortDirection** の値を "OF" に設定して、方向を "OF" にします。

最新順のソート方向にするには、次のようにします：

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'デフォルトのソートを最新順に変更する'; code-example-end]

**defaultSortDirection** の有効な値は次のとおりです：

- MR: "最新"
- NF: "最新順"
- OF: "最も古い順"

これはコードなしでも行えます。ウィジェットのカスタマイズページで「Default Sort Direction」セクションをご覧ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='「Default Sort Direction」セレクタは「最も関連性が高い」「最新順」「最も古い順」を提供します'; title='デフォルトのソート方向を変更する' app-screenshot-end]

各ページの各ソート方向のコメントは事前に計算されているため、すべてのソート方向で同じパフォーマンスになります。

---