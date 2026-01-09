此示例使用自定义代码以兼容 Wix。**您将无法使用其他教程中的 FastComments 代码片段。**

通过点击 `Enter Code` 并选择 `HTML` 打开表单以添加我们的自定义 HTML 对话框：

<div class="screenshot white-bg">
    <div class="title">步骤 3：打开 HTML 对话框</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="步骤 3：打开 HTML 对话框" />
</div>

复制以下 HTML 片段并粘贴到对话框中，然后点击 Update:

[inline-code-attrs-start title = 'Wix 评论代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">步骤 3：粘贴并保存</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="步骤 3：粘贴并保存" />
</div>

您现在应该会看到一个非常小的评论小部件版本：

<div class="screenshot white-bg">
    <div class="title">步骤 3：结果</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="步骤 3：结果" />
</div>

接下来我们将调整其位置和大小以适应页面。