WeeblyとFastCommentsの統合をうまく機能させるために、**2つ**の小さなコードスニペットを追加する必要があります。

最初のスニペットは Weebly の「コメントは閉じられています」メッセージを非表示にするためのもので、2つ目は実際に FastComments を読み込むためのものです。

まず、この小さなコードスニペットをコピーしてください：

[inline-code-attrs-start title = 'FastComments ヘッダー用コードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

次に、`Step One`で使った同じ設定ページで、`Post header code`の横にある`+`をクリックします。

<div class="screenshot white-bg">
    <div class="title">投稿ヘッダーコードを開く</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="投稿ヘッダーコードを開く" />
</div>

次のようにテキストボックスが開きます：

<div class="screenshot white-bg">
    <div class="title">投稿ヘッダーコードが開いた状態</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="投稿ヘッダーコードが開いた状態" />
</div>

次にコードスニペットを貼り付けます：

<div class="screenshot white-bg">
    <div class="title">ヘッダーコードスニペットを貼り付けた</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="ヘッダーコードスニペットを貼り付けた" />
</div>

次は FastComments を有効にするフッターコードです。`Post footer code`の横にあるプラス記号をクリックします：

<div class="screenshot white-bg">
    <div class="title">投稿フッターコードを開く</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="投稿フッターコードを開く" />
</div>

Weebly 用に**特別に**設計されたこのコードスニペットをコピーしてください：

[inline-code-attrs-start title = 'FastComments フッター用コードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // コメント表示ボタンを削除
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

次にフッターコードを貼り付けます：

<div class="screenshot white-bg">
    <div class="title">投稿フッターコードを追加した</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="投稿フッターコードを追加した" />
</div>

これで完了です！