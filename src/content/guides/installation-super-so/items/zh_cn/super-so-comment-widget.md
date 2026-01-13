---
## 在你的 Super.so Notion 文章中添加实时评论小部件

除了 Collab Chat，你还可以在 Notion 文章的底部添加传统的评论小部件。这样读者可以发表评论并就整篇文章进行讨论。

### 安装步骤

复制以下代码并将其粘贴到 Super.so 站点设置的 `Body` 部分：

[inline-code-attrs-start title = 'Super.so FastComments 实时评论小部件'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // 清理现有实例
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // 创建新的目标容器
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // 初始化 FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // 更新当前路径名
            currentPathname = window.location.pathname;
        }

        // 初始加载
        load();

        // 每 500 毫秒检查一次是否有更改
        setInterval(() => {
            // 如果路径名已更改则重新加载
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // 如果小部件被移除则重新加载
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // 如果容器被清空则重新加载
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### 重要说明

- 评论小部件将显示在你的 Notion 文章底部
- 每个页面根据 URL 路径获得自己的唯一评论线程
- 请确保将 `"demo"` 替换为你 FastComments 帐户中的实际 tenant ID
- 该小部件会自动处理 Super.so 的动态页面加载

---