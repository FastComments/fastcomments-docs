The Recent Discussionsウィジェットは、サイト上で最新のコメント活動のあったページを表示します。まだコメントが追加されているスレッドを強調表示するのに便利で、訪問者が静かなページにたどり着くのではなく、活発な会話に戻って参加できるようにします。

## Options

- **Title**（optional）：リストの上に表示される見出し。デフォルトは "Recent Discussions"。
- **Count**（optional）：表示するディスカッションの数。範囲は1から50。デフォルトは20。

## How to Add It

### Inside a Post or Page

In the block editor, add a **Shortcode** block and paste:

[inline-code-attrs-start title = '最近のディスカッション ショートコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### In a Sidebar or Footer (Classic Themes)

WordPress 管理画面で **外観 > ウィジェット** に移動します。ブロック挿入ツールから「FastComments」を検索し、**FastComments: Recent Discussions** を選択します。サイドバー、ヘッダー、またはフッター領域にドラッグし、ウィジェットパネルでタイトルとカウントを設定します。

### In a Block Theme (Full Site Editing)

**外観 > エディター** の下にある **サイトエディター** を開きます。ウィジェットを表示させたいテンプレートパーツに移動し、**Legacy Widget** ブロックを挿入して、ドロップダウンから **FastComments: Recent Discussions** を選択します。

## Troubleshooting

ウィジェットは、FastComments のセットアップが完了してテナント ID が保存された後にのみレンダリングされます。ウィジェット領域が空白の場合は、WordPress 管理画面の **FastComments** でセットアップを完了し、ページを再読み込みしてください。

ディスカッションの順序が古く見える場合は、基になるページが FastComments ダッシュボードで同期を完了しているか確認してください。ウィジェットはライブデータを読み込むため、最近インポートされたコメントが反映されるまでしばらく時間がかかることがあります。