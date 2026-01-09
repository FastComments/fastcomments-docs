[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

对于允许在初始页面加载后切换深色模式的网站，这会稍微复杂一些。

首先，评论小部件库的所有当前版本（React、Vue）在它们各自的代码仓库中都有切换深色模式的示例。

对于 VanillaJS 小部件，我们需要做更多工作。首先，FastCommentsUI 返回一个包含函数 "destroy" 和 "update" 的对象。

我们可以在每次想要更新评论小部件配置时简单地调用 update 函数，如下所示。下面是使用 VanillaJS 小部件切换深色模式的完整可运行示例。

[inline-code-attrs-start title = '切换深色模式的完整示例'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---