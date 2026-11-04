在下一步中，你需要复制下面预先制作好的小部件代码。

只要你已登录 FastComments.com，下面的代码片段就会自动包含你的账户信息。让我们复制它：

[inline-code-attrs-start title = 'Super.so FastComments 协作聊天代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // 清理已存在的实例
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // 清理已存在的顶部栏（如果有）
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // 创建新的顶部栏
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // 初始化 FastComments 协作聊天
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // 更新当前路径
            currentPathname = window.location.pathname;
        }

        // 初始加载
        load();

        // 每 500ms 检查一次变化
        setInterval(() => {
            // 如果路径变化则重新加载
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
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

现在将其粘贴到 `Body` 区域：

<div class="screenshot white-bg">
    <div class="title">粘贴的代码</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="粘贴的代码" />
</div>

如果你在粘贴代码后看到“这是一条演示消息”：

- 确保你已登录你的 fastcomments.com 账户。
- 确保你已启用第三方 Cookie。
- 然后刷新此页面并再次复制代码片段。此时 `tenantId` 应已填充为你租户的标识符。