---
次の手順では、以下の既成ウィジェットコードをコピーする必要があります。

FastComments.com にログインしている限り、以下のコードスニペットには既にアカウント情報が含まれています。コピーしましょう：

[inline-code-attrs-start title = 'Super.so FastComments コラボチャットのコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // 既存のインスタンスをクリーンアップ
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // 既存のトップバーが存在する場合はクリーンアップ
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // 新しいトップバーを作成
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // FastComments Collab Chat を初期化
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // 現在のパス名を更新
            currentPathname = window.location.pathname;
        }

        // 初期ロード
        load();

        // 変更を500msごとにチェック
        setInterval(() => {
            // パス名が変更された場合はリロード
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // ウィジェットが削除された場合はリロード
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // コンテナの内容が空になった場合はリロード
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">貼り付けたコード</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="貼り付けたコード" />
</div>

If you see a "this is a demo message" after pasting the code:

- fastcomments.com のアカウントにログインしていることを確認してください。
- サードパーティのクッキーが有効になっていることを確認してください。
- このページをリフレッシュし、コードスニペットを再度コピーしてください。`tenantId` があなたのテナント識別子で埋められているはずです。

---