在自定义代码选项卡的**页脚**部分，粘贴以下代码：

[inline-code-attrs-start title = 'Typeflo.io 实时评论代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">将代码粘贴到页脚部分</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="将 FastComments 代码粘贴到页脚部分" />
</div>

粘贴代码后，点击 **保存** 按钮以应用更改。

注意：此代码包含用于在您的 Typeflo.io 博客文章中动态将 FastComments 小部件放置到最佳位置的逻辑。其他代码片段无法与 Typeflo.io 的布局正常配合。

请记得在注册后将 `'demo'` 替换为您实际的 FastComments tenant ID。如果您已登录 FastComments.com，应该已经被替换。