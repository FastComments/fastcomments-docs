### 深色模式支援

### 動態深色模式

如果你網站的深色模式是透過在 body 元素新增 `.dark` 類別來控制，Collab Chat 的 UI 將會自動支援此功能而不需要重新初始化。小工具的樣式會設計為回應此類別的存在。

[inline-code-attrs-start title = '深色模式 CSS 範例'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* 你的深色模式 CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### 使用 CSS 自訂樣式

你可以使用 CSS 自訂高亮、聊天視窗和其他元素的外觀。小工具會新增特定的類別，讓你可以在樣式表中針對它們進行設定。

文字高亮使用 FastComments 的留言氣泡樣式系統，因此你對標準留言小工具所做的任何自訂也會影響 Collab Chat。

### 頂部列自訂

頂部列會顯示線上使用者數與討論數。你可以透過提供一個自訂元素作為 `topBarTarget` 來自訂它的位置：

[inline-code-attrs-start title = '自訂頂部列位置'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

或是將它完全停用，設定為 `null`：

[inline-code-attrs-start title = '停用頂部列'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### 行動裝置行為

在寬度小於 768px 的螢幕上，Collab Chat 會自動切換到行動裝置優化的版面。聊天視窗會以全螢幕顯示，而非浮動在文字旁邊，並且移除選取延遲以提供更即時的互動。

此行為為內建，不需要任何設定。小工具會自動偵測螢幕大小並做相應調整。

### 聊天視窗外觀

桌面版的聊天視窗寬度為 410px，並有一個 16px 的箭頭指向被標示的文字。視窗會根據可用的視口空間自動定位，使用像 `to-right`、`to-left`、`to-top` 與 `to-bottom` 這類定位類別。

你可以新增自訂 CSS 以調整這些視窗的顏色、字型、間距或其他視覺屬性。聊天視窗使用與標準 FastComments 小工具相同的元件結構，因此它們會繼承你套用的任何全域自訂。

### 在地化

Collab Chat 支援與標準 FastComments 小工具相同的所有在地化選項。設定 `locale` 選項以顯示不同語言的介面文字：

[inline-code-attrs-start title = '設定語系'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙語
});
[inline-code-end]

FastComments 支援數十種語言。locale 設定會影響所有介面文字，包括提示、按鈕與預設文字。

### 繼承的自訂選項

由於 Collab Chat 擴充了標準的留言小工具，它會繼承基礎小工具的所有自訂選項。這包括自訂 CSS 類別、自訂翻譯、頭像自訂、日期格式化等更多功能。

請參閱主要的 FastComments 自訂文件以取得完整的可用自訂選項清單。

### 使用自訂字型

如果你網站使用自訂字型，Collab Chat 的 UI 將會從你頁面的 CSS 繼承這些字型。如果你希望浮動聊天視窗使用相同的字型，你可能需要建立一個小工具自訂規則，並在該規則的自訂 CSS 中使用 `@import` 引入任何字型。

---