接下來我們將把 FastComments 小工具程式碼加入您的網站。此程式碼會搜尋所有標題為 `FastComments Goes Here` 的表單並將其替換為 FastComments。

所以在網站編輯器左下方點選 `Settings`：

<div class="screenshot white-bg">
    <div class="title">開啟設定</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="開啟設定" />
</div>

打開 `Custom Head Code` 區段：

<div class="screenshot white-bg">
    <div class="title">打開自訂頁首程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="打開自訂頁首程式碼" />
</div>

對於 Ionos 我們需要 FastComments 小工具程式碼的 **特殊版本**。來自 **其他教學** 的程式碼片段將無法使用。

現在複製以下程式碼：

[inline-code-attrs-start title = 'Ionos FastComments 程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                    // 取得不是全寬的元素
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

...然後依照顯示方式貼上並儲存：

<div class="screenshot white-bg">
    <div class="title">貼上並儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="貼上並儲存" />
</div>