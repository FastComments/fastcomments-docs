### Dark Mode Support

Image Chat 包含內建的深色模式支援。當您在設定中將 `hasDarkBackground: true` 設定時，聊天視窗與 UI 元素會自動調整，以便在深色背景上良好顯示。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

深色模式樣式會套用到聊天視窗、標記方塊，以及所有互動元素。如果您的網站有深色模式切換，您可以在模式改變時重新初始化 widget，或使用下方描述的 body class 方法。

### Dynamic Dark Mode

如果您網站的深色模式是透過在 body 元素新增 `.dark` 類別來控制，Image Chat 的 UI 會自動配合此類別，無需重新初始化。該 widget 的樣式設計會回應此類別的存在。

```css
/* 您的深色模式 CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Custom Styling with CSS

您可以使用 CSS 自訂標記、聊天視窗和其他元素的外觀。widget 會新增特定的類別，您可以在樣式表中針對這些類別進行調整。

聊天方塊與視窗使用 FastComments 的評論氣泡樣式系統，因此您對標準評論 widget 所做的任何自訂也會影響 Image Chat。

### Chat Square Sizing

`chatSquarePercentage` 選項控制可點擊標記的大小。預設為影像寬度的 5%：

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // 較大、更醒目的方塊
});
```

較小的數值會產生較不突出的標記，能與影像更好地融合。較大的數值則會讓標記更顯眼、也更容易點擊，尤其是在行動裝置或為了無障礙性考量時。

### Mobile Behavior

在寬度小於 768px 的螢幕上，Image Chat 會自動切換為行動裝置最佳化的版面配置。聊天視窗會以全螢幕方式顯示，而非在標記旁浮動，以在小螢幕上提供更好的可用性。

標記仍會保留在影像上其回應式的位置。使用者可以點擊任何標記以開啟全螢幕聊天介面。此行為為內建功能，無需任何額外設定。

### Chat Window Appearance

桌面上聊天視窗寬度為 300px，並有一個指向標記的 16px 箭頭。視窗會根據可用的視窗空間自動定位，使用像是 `to-right`、`to-left`、`to-top` 和 `to-bottom` 的定位類別。

您可以加入自訂 CSS 以調整這些視窗的顏色、字型、間距或其他視覺屬性。聊天視窗使用與標準 FastComments widget 相同的元件結構，因此會繼承您所套用的任何全域自訂。

### Lazy Initialization

聊天視窗會在桌面使用者滑鼠懸停時初始化，或在建立時立即初始化。這降低了初始載入的開銷，因為只有在使用者與標記互動時才會渲染聊天介面。

延遲初始化發生得很透明。使用者不會察覺到任何延遲，但如果影像上有許多標記，瀏覽器也不需要渲染數十個隱藏的聊天視窗。

### Localization

Image Chat 支援與標準 FastComments widget 相同的所有在地化選項。設定 `locale` 選項以在不同語言中顯示 UI 文字：

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // 法語
});
```

FastComments 支援數十種語言。locale 設定會影響所有 UI 文字，包括提示、按鈕和佔位文字。

### Inherited Customization Options

由於 Image Chat 擴充了標準評論 widget，它會繼承來自基礎 widget 的所有自訂選項。這包括自訂 CSS 類別、自訂翻譯、頭像自訂、日期格式化等等。

請參閱主要的 FastComments 自訂文件以取得可用自訂選項的完整清單。

### Working with Custom Fonts

如果您的網站使用自訂字型，Image Chat 的 UI 會從您頁面的 CSS 繼承這些字型。聊天視窗會在您的頁面 DOM 中渲染並遵循您現有的排版設定。

為取得最佳效果，請確保在初始化 Image Chat 之前載入自訂字型，或接受在字型載入期間可能出現的短暫無樣式文字閃爍。

### Marker Visual Design

方形標記具有細緻的視覺設計，使其在不搶走影像焦點的情況下仍然顯眼。如果您想要不同的視覺處理，可以使用 CSS 自訂它們的外觀。

標記包含滑鼠懸停狀態，在使用者將滑鼠移到標記上時提供回饋。在觸控裝置上，點擊互動會透過開啟聊天視窗立即提供回饋。