此範例使用自訂程式碼以與 Wix 相容。**您將無法使用其他教學中的 FastComments 程式碼片段。**

按一下 `Enter Code` 並選擇 `HTML` 來開啟表單以新增我們的自訂 HTML 對話方塊：

<div class="screenshot white-bg">
    <div class="title">步驟 3：開啟 HTML 對話方塊</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="步驟 3：開啟 HTML 對話方塊" />
</div>

將下列 HTML 片段複製並貼入對話方塊，然後按「更新」：

[inline-code-attrs-start title = 'Wix 留言 程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">步驟 3：貼上並儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="步驟 3：貼上並儲存" />
</div>

您現在應該會看到一個非常小的留言元件：

<div class="screenshot white-bg">
    <div class="title">步驟 3：結果</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="步驟 3：結果" />
</div>

接下來我們會調整其位置與大小以適合頁面。

---