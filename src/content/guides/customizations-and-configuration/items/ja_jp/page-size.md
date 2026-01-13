---
デフォルトでは、FastComments のページサイズは `30` です。スレッド内の返信も含まれます。

ページサイズは [ウィジェット設定 UI](https://fastcomments.com/auth/my-account/customize-widget) でカスタマイズでき、`10` から `200` の範囲で選択できます。

ページサイズを変更すると、アカウント内のすべてのコメントスレッドを再計算する必要があります。これには数分かかる場合があります。

ページはサーバー側で計算されるため、クライアント側のウィジェットでは設定できません。

以下に設定例を示します:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.page-size'; title='Custom Page Sizes' app-screenshot-end]

ページサイズはグローバル、ドメインごと、ページごとに異なるカスタマイズルールを作成することで変更できます。

これにより、当社プラットフォームを通じてコメントを表示するために使用しているすべてのクライアント、統合、およびフレームワークに影響します。

---