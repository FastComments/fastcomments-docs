現在我們要為你產生自訂的 FastComments 程式碼。使用下面的精靈來設定 FastComments 在你的 GoHighLevel 網站上的運作方式：

[snippet id="gohighlevel-wizard"]

### 不同的留言框類型

你可以設定 `TYPE = 'commenting'` 這一行來切換所使用的產品（例如你可以改成 `live` 用於串流聊天，或 `collab` 用於協作聊天）。

### 將留言框放在你想要的位置

假設你想把留言框放在頁面中特定的位置，而不是預設位置。  
將這一行改成：

    const TARGET_ELEMENT_ID = ''; // 設為使用目標 div 模式

改成：

    const TARGET_ELEMENT_ID = 'fc_box'; // 設為使用目標 div 模式

接著在 GHL 編輯器中，點選 "code" 按鈕，並加入你想要放置留言的位置：

[inline-code-attrs-start title = 'GoHighLevel FastComments Div 元素'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### 每頁不同的留言框類型

假設你想讓使用者標註並討論部分文字，或改用串流聊天 UI。

首先請依照上面「將留言框放在你想要的位置」的步驟。

注意在那個小程式碼片段中有 `type="commenting"`。

例如若你想啟用 collab 聊天，將 type 改為 `type="collab"`。

### 僅在特定頁面顯示

如果你沒有設定 `TARGET_ELEMENT_ID`，你可以改為設定 `VALID_PATTERNS` 變數，用以設定留言應該顯示的 URL 路徑。預設情況下，它會在 URL 中包含 `/post` 的頁面顯示。

### 設定 Collab 聊天

你可以設定 collab 聊天僅在特定區域內的 HTML 周圍加入協作功能。例如，假設你在 footer 加上上述程式碼，然後在文章/頁面內容中加入此 div 來啟用 collab 聊天：

[inline-code-attrs-start title = '對指定內容的 Collab 聊天'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

那麼 `<div>` 內的段落元素就會啟用 collab 聊天，而頁面上的其他部分則不會。如果你沒有在 `<div>` 中放任何內容，它就會在整個文章主體上啟用 collab 聊天。

---