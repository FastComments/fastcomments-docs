在下一步，您需要複製下方預先製作的小工具程式碼。

只要您已在 FastComments.com 登入，以下程式碼片段就會自動包含您的帳戶資訊。現在來複製它：

[inline-code-attrs-start title = 'Super.so FastComments 協作聊天程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // 清除現有的實例
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // 若存在則清除現有的頂部列
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // 建立新的頂部列
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // 初始化 FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // 更新目前的 pathname
            currentPathname = window.location.pathname;
        }

        // 初始載入
        load();

        // 每 500ms 檢查變更
        setInterval(() => {
            // 若 pathname 改變則重新載入
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // 若小工具被移除則重新載入
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // 若容器被清空則重新載入
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
    <div class="title">已貼上程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="已貼上程式碼" />
</div>

If you see a "this is a demo message" after pasting the code:

- 確保您已登入您的 fastcomments.com 帳戶。
- 確保已啟用第三方 Cookie。
- 然後重新整理此頁面並再次複製程式碼片段。它應該會在 `tenantId` 中填入您租戶的識別碼。