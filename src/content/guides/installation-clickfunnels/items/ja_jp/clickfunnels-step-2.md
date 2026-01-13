---
Now that we're in the template editor, we must decide where we want to display the comments, or live chat.

テンプレートエディタに入ったので、コメント（ライブチャット）を表示する場所を決める必要があります。

In this example we will add it directly below the video. Hover the element to add the widget to the end of, and click `ADD ELEMENT`:

この例ではビデオの直下に追加します。ウィジェットを追加したい要素にカーソルを合わせ、`ADD ELEMENT`をクリックします：

<div class="screenshot white-bg">
    <div class="title">Add Element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Add Element" />
</div>

<div class="screenshot white-bg">
    <div class="title">要素を追加</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="要素を追加" />
</div>

Select `CUSTOM JS/HTML`:

`CUSTOM JS/HTML` を選択します：

<div class="screenshot white-bg">
    <div class="title">Select CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Select CUSTOM JS/HTML" />
</div>

<div class="screenshot white-bg">
    <div class="title">CUSTOM JS/HTML を選択</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="CUSTOM JS/HTML を選択" />
</div>

Now let's open the code editor where we'll paste our code.

次に、コードを貼り付けるコードエディタを開きます。

ClickFunnels is a bit confusing on this next step.

ClickFunnels は次の手順が少し分かりにくいです。

It's important that you *DO NOT* select `Code` when you hover over the new element. Instead, select `SETTINGS`:

新しく追加した要素にカーソルを合わせたときに `Code` を選択しないことが重要です。代わりに `SETTINGS` を選択してください：

<div class="screenshot white-bg">
    <div class="title">Select SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Select SETTINGS" />
</div>

<div class="screenshot white-bg">
    <div class="title">SETTINGS を選択</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="SETTINGS を選択" />
</div>

Now on the right hand side we can click `Open Code Editor`:

右側のパネルで `Open Code Editor` をクリックします：

<div class="screenshot white-bg">
    <div class="title">Click Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Click Open Code Editor" />
</div>

<div class="screenshot white-bg">
    <div class="title">Open Code Editor をクリック</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Open Code Editor をクリック" />
</div>

You'll see a big square open up. This is where we can paste our code. Copy the following snippet (use the copy button in the top right):

大きな四角いウィンドウが開きます。ここにコードを貼り付けます。以下のスニペットをコピーしてください（右上のコピーボタンを使用してください）：

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
            // 一部のプロバイダはコードスニペットを非同期に変更することがあります
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

This code snippet is for our Streaming Chat product, which goes well with videos. If you want the Live Commenting widget code snippet instead, which
goes best with regular pages or blog posts, it's at the end of this tutorial.

このコードスニペットは動画に適した Streaming Chat 製品用です。代わりに Live Commenting ウィジェットのコードスニペットが必要で、通常のページやブログ投稿に最適な場合は、
チュートリアルの最後にあります。

When we paste the code snippet into the window, it should look like this:

コードスニペットをウィンドウに貼り付けると、次のようになります：

<div class="screenshot white-bg">
    <div class="title">Paste Code</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Paste Code" />
</div>

<div class="screenshot white-bg">
    <div class="title">コードを貼り付け</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="コードを貼り付け" />
</div>

Now we just have to close the box:

あとはボックスを閉じるだけです：

<div class="screenshot white-bg">
    <div class="title">Close</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Close" />
</div>

<div class="screenshot white-bg">
    <div class="title">閉じる</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="閉じる" />
</div>

Now you can preview your changes! Feel free to move the widget around and see where you like it best.

これで変更をプレビューできます。ウィジェットを移動して、最適な位置を確認してください。

<div class="screenshot white-bg">
    <div class="title">Preview</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Preview" />
</div>

<div class="screenshot white-bg">
    <div class="title">プレビュー</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="プレビュー" />
</div>

Success! Don't forget to test mobile!

成功です！モバイルでのテストをお忘れなく！

<div class="screenshot white-bg">
    <div class="title">Success!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Success!" />
</div>

<div class="screenshot white-bg">
    <div class="title">成功!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="成功!" />
</div>

---