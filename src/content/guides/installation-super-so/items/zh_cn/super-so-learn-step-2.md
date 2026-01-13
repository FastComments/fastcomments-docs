在下一步，您需要复制下面预先制作的小部件代码。

只要您已登录 FastComments.com，下面的代码片段就会包含您的帐户信息。现在复制它：

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

            // 清理现有实例
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // 如果存在则清理现有顶部栏
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

            // 更新当前路径名
            currentPathname = window.location.pathname;
        }

        // 初始加载
        load();

        // 每 500ms 检查是否有变化
        setInterval(() => {
            // 如果路径名改变则重新加载
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

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">已粘贴的代码</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="已粘贴的代码" />
</div>

If you see a "这是演示消息" after pasting the code:

- Ensure you're logged into your fastcomments.com account.
- Ensure you have 3rd party cookies enabled.
- Then refresh this page and copy the code snippet again. It should have `tenantId` populated with your tenant's identifier.

---