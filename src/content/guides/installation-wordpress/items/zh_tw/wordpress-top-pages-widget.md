熱門頁面小工具會顯示您網站上留言最多的頁面。它有助於向新訪客呈現最具互動性的內容並增加網站停留時間。

## 選項

- **標題**（可選）：顯示在列表上方的標題。預設為「熱門頁面」。

熱門頁面小工具會根據可用資料自動選擇版面配置，且不接受 count 屬性。

## 如何新增

### 在文章或頁面內

在區塊編輯器中，加入一個 **Shortcode（短代碼）** 區塊，然後貼上：

[inline-code-attrs-start title = '熱門頁面 短代碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### 在側邊欄或頁尾（經典主題）

在 WordPress 管理介面中前往 **外觀 > 小工具**。使用區塊插入器搜尋 "FastComments" 並選擇 **FastComments: 熱門頁面**。將它拖放到側邊欄、頁首或頁尾位置，然後在小工具面板中設定標題。

### 在區塊主題（整站編輯）

在 **外觀 > 編輯器** 下開啟 **網站編輯器**。前往要放置小工具的範本區塊，插入一個 **Legacy Widget（舊版小工具）** 區塊，然後從下拉選單中選擇 **FastComments: 熱門頁面**。

## 疑難排解

只有在完成 FastComments 設定並儲存了 tenant ID 之後，小工具才會呈現。如果小工具區域是空白的，請在 WordPress 管理後台的 **FastComments** 頁面完成設定，然後重新載入該頁面。