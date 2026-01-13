接下來，我們要設定讓留言串依據目前頁面變化，讓使用者可以針對當前顯示的內容進行討論。

如果不進行以下步驟，你的整個網站只會有一個全域留言串 —— 這樣沒什麼用處。

#### 開發模式

要新增這個功能，我們需要進入 Wix 所稱的 `Dev Mode`。

點選畫面上方的 `Dev Mode` 選項。

<div class="screenshot white-bg">
    <div class="title">啟用開發模式</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="啟用開發模式" />
</div>

#### 設定元素 ID

我們將加入自訂程式碼來完成此功能，但首先需要為新的嵌入元素指定一個 ID 以便參照。

我們把它命名為 `fastcomments`。

點選我們新增的嵌入元素，在開發模式中在右下角你應該會看到一個像 `html1` 的 ID 欄位：

<div class="screenshot white-bg">
    <div class="title">ID 欄位</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID 欄位" />
</div>

將它改為 `fastcomments` 然後按 Enter：

<div class="screenshot white-bg">
    <div class="title">設定 ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="設定 ID" />
</div>

現在我們可以加入自訂程式碼來告訴留言區我們正在查看哪一個頁面。

在畫面底部你應該會看到類似這樣的程式碼編輯器：

<div class="screenshot white-bg">
    <div class="title">開啟編輯器</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="開啟編輯器" />
</div>

複製下列程式碼並貼到那裡：

[inline-code-attrs-start title = 'Wix 評論導覽程式碼片段'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">加入導覽程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="加入導覽程式碼" />
</div>

---