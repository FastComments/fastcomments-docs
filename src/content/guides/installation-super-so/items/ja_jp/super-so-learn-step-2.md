次のステップでは、以下の事前作成済みウィジェットコードをコピーする必要があります。

FastComments.com にログインしていれば、以下のコードスニペットにはすでにあなたのアカウント情報が含まれています。コピーしましょう:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat コード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Clean up existing instance
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Clean up existing top bar if it exists
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Create new top bar
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Initialize FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Update current pathname
            currentPathname = window.location.pathname;
        }

        // Initial load
        load();

        // Check every 500ms for changes
        setInterval(() => {
            // Reload if pathname changed
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Reload if widget was removed
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Reload if container was emptied
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

次に `Body` エリアに貼り付けてください:

<div class="screenshot white-bg">
    <div class="title">貼り付けたコード</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="貼り付けたコード" />
</div>

コードを貼り付けた後に「this is a demo message」というメッセージが表示される場合は:

- fastcomments.com のアカウントにログインしていることを確認してください。
- サードパーティの Cookie が有効になっていることを確認してください。
- その後、このページを更新してコードスニペットを再度コピーしてください。`tenantId` にあなたのテナントの識別子が設定されているはずです。