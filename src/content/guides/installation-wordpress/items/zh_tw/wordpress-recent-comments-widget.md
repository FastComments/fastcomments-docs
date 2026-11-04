「最近留言」小工具會顯示在您整個網站上發布的最新留言。它適用於側邊欄、頁尾，或任何您想展示最新動態以鼓勵進一步閱讀的地方。

## 選項

- **標題**（選填）：顯示在列表上方的標題。預設為「Recent Comments」。
- **數量**（選填）：要顯示多少則留言。範圍為 1 到 50。預設為 5。

## 如何新增

### 在文章或頁面內

在區塊編輯器中，新增一個 **Shortcode** 區塊並貼上：

[inline-code-attrs-start title = '最近留言短代碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

`count` 屬性可接受 1 到 50 之間的任何值。

### 在側邊欄或頁尾（傳統佈景主題）

前往 WordPress 管理後台的 **外觀 > 小工具**。在區塊插入器中搜尋「FastComments」，並選擇 **FastComments: Recent Comments**。將它拖曳到側邊欄、頁首或頁尾區域，然後從小工具面板設定標題與數量。

### 在區塊佈景主題中（全站編輯）

開啟 **外觀 > 編輯器** 下的 **網站編輯器**。導覽至小工具應出現的範本部分，插入一個 **Legacy Widget** 區塊，並從下拉選單中選擇 **FastComments: Recent Comments**。

## 疑難排解

小工具只有在 FastComments 設定完成並儲存租戶 ID 後才會顯示。如果小工具區域是空白的，請在 WordPress 管理後台的 **FastComments** 下完成設定，然後重新載入頁面。