現在我們進入樣板編輯器後，需要決定要在何處顯示評論或即時聊天。

在此範例中，我們會將它直接新增於影片下方。將游標移到該元素以將小工具新增到末端，然後點擊 `ADD ELEMENT`：

<div class="screenshot white-bg">
    <div class="title">新增元素</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="新增元素" />
</div>

選擇 `CUSTOM JS/HTML`：

<div class="screenshot white-bg">
    <div class="title">選擇 CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="選擇 CUSTOM JS/HTML" />
</div>

現在打開程式碼編輯器，我們會在那裡貼上程式碼。

在下一個步驟，ClickFunnels 會有點令人困惑。

當你將游標移到新元素上時，務必*不要*選擇 `Code`，而是選擇 `SETTINGS`：

<div class="screenshot white-bg">
    <div class="title">選擇 SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="選擇 SETTINGS" />
</div>

現在在右側我們可以點擊 `Open Code Editor`：

<div class="screenshot white-bg">
    <div class="title">點擊 Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="點擊 Open Code Editor" />
</div>

你會看到一個大方塊打開。這裡就是我們可以貼上程式碼的地方。複製以下片段（使用右上角的複製按鈕）：

[inline-code-attrs-start title = 'ClickFunnels 串流聊天程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 某些提供者會把程式碼片段改成非同步
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

此程式碼片段適用於我們的 Streaming Chat（串流聊天）產品，非常適合搭配影片。如果你想使用 Live Commenting（即時評論）小工具的程式碼片段，它比較適合一般頁面或部落格文章，該片段位於本教學的結尾。

當我們把程式碼片段貼到視窗中時，應該會像這樣：

<div class="screenshot white-bg">
    <div class="title">貼上程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="貼上程式碼" />
</div>

現在我們只要關閉視窗：

<div class="screenshot white-bg">
    <div class="title">關閉</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="關閉" />
</div>

現在你可以預覽變更！隨意移動小工具，找出你最喜歡的位置。

<div class="screenshot white-bg">
    <div class="title">預覽</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="預覽" />
</div>

成功！別忘了測試行動裝置！

<div class="screenshot white-bg">
    <div class="title">成功！</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="成功！" />
</div>