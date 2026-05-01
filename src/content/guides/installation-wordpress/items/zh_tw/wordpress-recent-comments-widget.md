The Recent Comments widget displays the most recent comments posted across your entire site. It's useful in sidebars, footers, or anywhere you want to surface fresh activity to encourage further reading.

## 選項

- **標題**（可選）：顯示在列表上方的標題。預設為 "最新留言"。
- **數量**（可選）：要顯示的留言數目。範圍為 1 到 50。預設為 5。

## 如何加入

### 在文章或頁面內

在區塊編輯器中，新增一個 **Shortcode** 區塊並貼上：

[inline-code-attrs-start title = '最新留言 短代碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

屬性 `count` 接受介於 1 到 50 之間的任何值。

### 在側邊欄或頁腳（經典佈景主題）

在 WordPress 管理後台前往 **外觀 > 小工具**。從區塊插入器搜尋「FastComments」，並選擇 **FastComments: Recent Comments**。將它拖放到側邊欄、頁首或頁腳區域，然後從小工具面板設定標題和數量。

### 在區塊佈景主題（全站編輯）

在 **外觀 > 編輯器** 下開啟 **網站編輯器**。導航到要顯示小工具的樣板區段，插入一個 **Legacy Widget** 區塊，並從下拉選單選擇 **FastComments: Recent Comments**。

## 疑難排解

小工具只有在完成 FastComments 設定並儲存 tenant ID 後才會顯示。如果小工具區域為空白，請在 WordPress 管理後台的 **FastComments** 中完成設定，然後重新載入頁面。