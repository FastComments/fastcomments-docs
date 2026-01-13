---
## Super.so の Notion 記事にライブコメントウィジェットを追加する

Collab Chat に加えて、Notion 記事の下部に従来型のコメントウィジェットを追加できます。これにより、読者が記事全体に対してコメントを残したり議論を行ったりできます。

### インストール手順

以下のコードをコピーして、Super.so のサイト設定の `Body` セクションに貼り付けてください:

[inline-code-attrs-start title = 'Super.so FastComments ライブコメントウィジェット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // 既存のインスタンスをクリーンアップ
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // 新しいターゲットを作成
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // FastComments を初期化
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // 現在のパス名を更新
            currentPathname = window.location.pathname;
        }

        // 初回ロード
        load();

        // 変更を500msごとにチェック
        setInterval(() => {
            // パス名が変わったらリロード
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // ウィジェットが削除されたらリロード
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // コンテナが空になったらリロード
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### 重要な注意点

- コメントウィジェットは Notion 記事の下部に表示されます
- 各ページは URL パスに基づいた固有のコメントスレッドを持ちます
- FastComments アカウントの実際のテナントID で "demo" を置き換えてください
- ウィジェットは Super.so の動的ページ読み込みを自動で処理します

---