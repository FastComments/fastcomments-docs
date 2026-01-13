### クイックスタート

Collab Chat の開始は簡単です。必要なのは FastComments Collab Chat スクリプト、注釈したいテキストを含む HTML 要素、および Tenant ID を含む設定オブジェクトだけです。

### インストール

ページに Collab Chat スクリプトを追加します:

[inline-code-attrs-start title = 'Collab Chat スクリプトの読み込み'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### 基本的な実装

最小限の例は次のとおりです:

[inline-code-attrs-start title = '基本的な Collab Chat 実装'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- コンテンツコンテナ -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Collab Chat スクリプトを読み込む -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Collab Chat を初期化する -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

実際の FastComments Tenant ID がまだ 'demo' でない場合は、それを実際の Tenant ID に置き換えてください。Tenant ID は [FastComments ダッシュボード](https://fastcomments.com/auth/my-account/api-secret) で確認できます。

### 仕組み

初期化が完了すると、ユーザーはターゲット要素内の任意のテキストを選択できます。短い遅延の後（デスクトップでは約 3.5 秒）、ディスカッションを開始できるプロンプトが表示されます。ディスカッションが作成されると、選択したテキストに視覚的なハイライトが表示されます。他のユーザーはそのハイライトにカーソルを合わせるかクリックしてディスカッションを表示・参加できます。すべてのディスカッションはすべての訪問者間でリアルタイムに同期されます。

### ライブデモ

Collab Chat の動作は当社の [ライブデモページ](https://fastcomments.com/product/collab-chat) でご覧いただけます。

### 次のステップ

基本が動作するようになったら、Configuration Options ガイドで外観や動作をカスタマイズできます。Text Selection Behavior ガイドでテキスト選択の仕組みを確認してください。カスタマイズとダークモードのサポートについては Customization ガイドをご覧ください。高度な統合については API Reference を参照してください。

### フロントエンドライブラリ

すべての FastComments フロントエンドライブラリ（react, vue, angular など）に Collab Chat が含まれています。