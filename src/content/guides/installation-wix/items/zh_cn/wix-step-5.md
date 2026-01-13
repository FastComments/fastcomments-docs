接下来，我们需要设置，使评论线程根据当前页面变化，从而允许用户讨论当前显示的内容。

如果不执行以下步骤，你的网站将只有一个全局评论线程——这并不太实用。

#### 开发者模式

要添加此功能，我们需要进入 Wix 所称的 `Dev Mode`。

点击屏幕顶部的 `Dev Mode` 选项。

<div class="screenshot white-bg">
    <div class="title">启用 Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="启用 Dev Mode" />
</div>

#### 设置元素 ID

我们将添加自定义代码来实现此功能，但首先需要为新嵌入元素设置一个用于引用的 ID。

我们把它命名为 `fastcomments`。

点击我们添加的新嵌入元素，在开发者模式下右下角你应该能看到一个 ID 字段，其值类似 `html1`：

<div class="screenshot white-bg">
    <div class="title">ID 字段</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID 字段" />
</div>

把它改为 `fastcomments` 然后按回车：

<div class="screenshot white-bg">
    <div class="title">设置 ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="设置 ID" />
</div>

现在我们可以添加自定义代码，告诉评论区域当前正在查看哪个页面。

在屏幕底部你应该能看到如下的代码编辑器：

<div class="screenshot white-bg">
    <div class="title">打开编辑器</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="打开编辑器" />
</div>

复制以下代码并粘贴到其中：

[inline-code-attrs-start title = 'Wix 评论导航代码片段'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">添加导航代码</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="添加导航代码" />
</div>

---