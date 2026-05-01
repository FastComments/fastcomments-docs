The Recent Discussions 小工具會顯示您網站上最近有評論活動的頁面。它有助於突顯仍在持續被回覆的討論串，讓訪客能直接回到活躍的對話，而不是進入較為冷清的頁面。

## 選項

- **Title**（可選）：顯示在列表上方的標題。預設為「最近討論」。
- **Count**（可選）：要顯示的討論數量。範圍為 1 到 50。預設為 20。

## 如何新增

### 在文章或頁面內

在區塊編輯器中，新增一個 **短代碼（Shortcode）** 區塊並貼上：

[inline-code-attrs-start title = '最近討論 短代碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

`count` 屬性接受 1 到 50 之間的任何值。

### 在側邊欄或頁尾（傳統佈景主題）

在 WordPress 管理介面前往 **外觀 > 小工具**。從區塊插入器中搜尋 "FastComments"，並選擇 **FastComments: Recent Discussions**。將它拖到側邊欄、頁首或頁尾區域，然後在小工具面板中設定標題與數量。

### 在區塊佈景（完整網站編輯）

在 **外觀 > 編輯器** 下開啟 **網站編輯器**。導覽到小工具應出現的範本區段，插入一個 **Legacy Widget** 區塊，並從下拉選單中選擇 **FastComments: Recent Discussions**。

## 疑難排解

小工具只有在完成 FastComments 設定並儲存了 tenant ID 後才會呈現。如果小工具區域為空，請在 WordPress 管理介面中的 **FastComments** 完成設定後重新載入頁面。

如果討論排序看起來較為過時，請確認相關頁面是否已在 FastComments 儀表板完成同步。該小工具會讀取即時資料，因此新匯入的評論可能需要一些時間才會顯示。