テンプレートエディタに入ったので、コメントまたはライブチャットを表示する場所を決める必要があります。

この例では、動画のすぐ下に追加します。ウィジェットを末尾に追加したい要素にカーソルを合わせ、`ADD ELEMENT` をクリックします:

<div class="screenshot white-bg">
    <div class="title">要素を追加</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="要素を追加" />
</div>

`CUSTOM JS/HTML` を選択します:

<div class="screenshot white-bg">
    <div class="title">CUSTOM JS/HTML を選択</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="CUSTOM JS/HTML を選択" />
</div>

次に、コードを貼り付けるコードエディタを開きましょう。

ClickFunnels は次の手順が少し分かりにくくなっています。

新しい要素にカーソルを合わせたときに `Code` を選択しないことが *重要* です。代わりに `SETTINGS` を選択してください:

<div class="screenshot white-bg">
    <div class="title">SETTINGS を選択</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="SETTINGS を選択" />
</div>

次に、右側で `Open Code Editor` をクリックします:

<div class="screenshot white-bg">
    <div class="title">Open Code Editor をクリック</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Open Code Editor をクリック" />
</div>

大きな四角い領域が開きます。ここにコードを貼り付けます。次のスニペットをコピーしてください (右上のコピーボタンを使用します):

[inline-code-attrs-start title = 'ClickFunnels ストリーミングチャットのコードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 一部のプロバイダーはコードスニペットを非同期に変更します
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

このコードスニペットは、動画と相性の良い Streaming Chat 製品用のものです。通常のページやブログ記事に最も適した Live Commenting ウィジェットの
コードスニペットが必要な場合は、このチュートリアルの最後にあります。

コードスニペットをウィンドウに貼り付けると、次のようになります:

<div class="screenshot white-bg">
    <div class="title">コードを貼り付け</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="コードを貼り付け" />
</div>

あとはボックスを閉じるだけです:

<div class="screenshot white-bg">
    <div class="title">閉じる</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="閉じる" />
</div>

これで変更をプレビューできます! ウィジェットを自由に移動して、最適な位置を確認してください。

<div class="screenshot white-bg">
    <div class="title">プレビュー</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="プレビュー" />
</div>

成功です! モバイルでのテストもお忘れなく!

<div class="screenshot white-bg">
    <div class="title">成功!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="成功!" />
</div>
