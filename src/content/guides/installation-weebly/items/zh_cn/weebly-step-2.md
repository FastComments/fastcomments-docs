为了让 Weebly 和 FastComments 的集成顺利工作，我们需要添加 **两个** 小的代码片段。

第一个片段用于隐藏 Weebly “Comments are Closed” 消息，第二个片段用于实际加载 FastComments。

首先，复制这段小代码片段：

[inline-code-attrs-start title = 'FastComments 头部代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

然后，在与 `Step One` 相同的设置页面上，点击 `Post header code` 旁边的 `+`。

<div class="screenshot white-bg">
    <div class="title">打开文章头部代码</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="打开文章头部代码" />
</div>

你应该会看到一个文本框像这样打开：

<div class="screenshot white-bg">
    <div class="title">文章头部代码已打开</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="文章头部代码已打开" />
</div>

现在把我们的代码片段粘贴进来：

<div class="screenshot white-bg">
    <div class="title">头部代码片段已粘贴</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="头部代码片段已粘贴" />
</div>

接下来是用于启用 FastComments 的页脚代码。点击 `Post footer code` 旁边的加号：

<div class="screenshot white-bg">
    <div class="title">打开文章页脚代码</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="打开文章页脚代码" />
</div>

复制这段专为 Weebly 设计的代码片段：

[inline-code-attrs-start title = 'FastComments 页脚代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // remove show comments button
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

现在把页脚代码粘贴进去：

<div class="screenshot white-bg">
    <div class="title">文章页脚代码已添加</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="文章页脚代码已添加" />
</div>

就完成了！