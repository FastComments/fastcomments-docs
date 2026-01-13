## 在您的 Super.so Notion 文章中新增即時留言小工具

除了 Collab Chat 之外，您可以在 Notion 文章底部新增傳統留言小工具。讀者可以針對整篇文章留下留言並進行討論。

### 安裝步驟

複製下列程式碼並貼到您 Super.so 網站設定的 `Body` 區段：

[inline-code-attrs-start title = 'Super.so FastComments 即時留言小工具'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // 清除現有實例
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // 建立新的目標元素
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // 初始化 FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
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
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### 重要說明

- 留言小工具會顯示在您的 Notion 文章底部
- 每個頁面會根據 URL 路徑擁有自己的唯一留言串
- 請務必將 `"demo"` 替換為您 FastComments 帳戶中的實際 tenant ID
- 小工具會自動處理 Super.so 的動態頁面載入

---