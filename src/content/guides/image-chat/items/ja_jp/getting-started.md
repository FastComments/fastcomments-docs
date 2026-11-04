### ユースケース

Image Chatは、チームがモックアップやスクリーンショット内の特定の要素について話し合う必要があるデザインフィードバックに最適です。製品レビューサイトでは、製品写真に写っている特定の機能について顧客が話し合えるようにできます。教育プラットフォームでは、図、地図、科学画像についての議論に利用できます。フォトギャラリーは、場所固有のコメントでインタラクティブにできます。不動産サイトでは、物件写真に写っている特定の特徴について閲覧者が話し合えるようになります。

### クイックスタート

Image Chatの導入は簡単です。FastCommentsのImage Chatスクリプト、画像を含むimage要素またはコンテナ、そしてテナントIDを含む設定オブジェクトが必要です。

### インストール

Image Chatスクリプトをページに追加します:

[inline-code-attrs-start title = 'Image Chatスクリプトの読み込み'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### 基本的な実装

最小限の例を以下に示します:

[inline-code-attrs-start title = '基本的なImage Chatの実装'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Your image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialize Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

まだ置き換えていない場合は、`'demo'` を実際のFastCommentsテナントIDに置き換えてください。テナントIDは[FastCommentsダッシュボード](https://fastcomments.com/auth/my-account)で確認できます。

### 仕組み

初期化すると、ユーザーは画像上の任意の場所をクリックできます。クリックが発生すると、その位置に視覚的な四角形のマーカーが表示され、チャットウィンドウが開きます。他のユーザーはすべてのマーカーを見ることができ、マーカーをクリックしてその議論を閲覧または参加できます。すべての議論は、すべての訪問者間でリアルタイムに同期されます。

このウィジェットはパーセンテージベースの位置指定を使用しているため、画像がリサイズされたり、異なる画面サイズで表示されたりしても、マーカーは正しい位置に保たれます。

### ライブデモ

Image Chatの動作は、[ライブデモページ](https://fastcomments.com/product/image-chat)で確認できます。

### 次のステップ

基本が動作するようになったら、Configuration Optionsガイドで外観と動作をカスタマイズできます。Responsive Designガイドで、パーセンテージベースの位置指定の仕組みを確認してください。Customizationガイドで、スタイリングとダークモードのサポートについて学びましょう。高度な統合については、APIリファレンスをご覧ください。

### フロントエンドライブラリ

すべてのFastCommentsフロントエンドライブラリ(react、vue、angularなど)にImage Chatが含まれています。