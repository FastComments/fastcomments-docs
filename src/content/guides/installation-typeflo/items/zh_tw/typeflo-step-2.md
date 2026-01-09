在「自訂程式碼」分頁的 **頁尾** 區段，貼上以下程式碼：

[inline-code-attrs-start title = 'Typeflo.io 即時留言程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">將程式碼貼入頁尾區段</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="在頁尾區段貼上 FastComments 程式碼" />
</div>

貼上程式碼後，按 **儲存** 按鈕以套用變更。

注意：此程式碼包含邏輯，可動態將 FastComments 小工具放置在 Typeflo.io 部落格文章的最佳位置。其他程式碼片段在 Typeflo.io 的版面配置下可能無法正常運作。

請記得在註冊後將 `'demo'` 替換為您實際的 FastComments tenant ID。如果您已登入 FastComments.com，它應該已經被替換。