### ユースケース

Image Chatは、チームがモックアップやスクリーンショットの特定要素について議論するデザインフィードバックに最適です。製品レビューサイトでは、顧客が商品写真に写っている特定の機能について議論できます。教育プラットフォームでは図、地図、科学画像の議論に利用できます。フォトギャラリーは位置特定のコメントでインタラクティブになります。不動産サイトでは物件写真に写る特定の特徴について閲覧者が議論できます。

### クイックスタート

Image Chatの開始は簡単です。必要なのは FastComments Image Chat スクリプト、画像を含む img 要素またはコンテナ、そして Tenant ID を含む設定オブジェクトです。

### インストール

ページにImage Chatスクリプトを追加します:

[inline-code-attrs-start title = 'Image Chatスクリプトの読み込み'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### 基本的な実装

最小限の例を示します:

[inline-code-attrs-start title = '基本的な Image Chat の実装'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- あなたの画像 -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Image Chatスクリプトを読み込む -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Image Chat を初期化 -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments ダッシュボード](https://fastcomments.com/auth/my-account).

### 仕組み

初期化されると、ユーザーは画像の任意の場所をクリックできます。クリックが発生すると、その位置に四角い視覚マーカーが表示され、チャットウィンドウが開きます。他のユーザーはすべてのマーカーを見て、それらをクリックして議論を閲覧したり参加したりできます。すべての議論はすべての訪問者間でリアルタイムに同期されます。

ウィジェットはパーセンテージベースの位置指定を使用するため、画像がリサイズされたり異なる画面サイズで表示されたりしても、マーカーは正しい位置にとどまります。

### ライブデモ

Image Chatの動作は[ライブデモページ](https://fastcomments.com/product/image-chat)で確認できます。

### 次のステップ

基本が動作したら、Configuration Options ガイドで外観や挙動をカスタマイズできます。Responsive Design ガイドを参照して、パーセンテージベースの位置指定の動作を理解してください。Customization ガイドでスタイリングやダークモードのサポートについて学んでください。高度な統合については、API リファレンスを参照してください。

### フロントエンドライブラリ

すべての FastComments フロントエンドライブラリ（react、vue、angular など）には Image Chat が含まれています。