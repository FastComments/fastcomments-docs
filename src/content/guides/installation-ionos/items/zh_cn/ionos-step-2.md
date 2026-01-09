接下来我们将把 FastComments 小部件代码添加到你的网站。该代码将搜索所有标题为 `FastComments Goes Here` 的表单，并将其替换为 FastComments。

所以现在在站点编辑器的左下角进入 `Settings`：

<div class="screenshot white-bg">
    <div class="title">打开设置</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="打开设置" />
</div>

打开 `Custom Head Code` 部分：

<div class="screenshot white-bg">
    <div class="title">打开自定义头部代码</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="打开自定义头部代码" />
</div>

对于 Ionos，我们需要 FastComments 小部件代码的 **特殊版本**。来自 **其他教程** 的代码片段 **将无法使用。**

现在复制以下代码：

[inline-code-attrs-start title = 'Ionos FastComments 代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // 获取非全宽的元素
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...并按如下所示粘贴：

<div class="screenshot white-bg">
    <div class="title">粘贴并保存</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="粘贴并保存" />
</div>

---