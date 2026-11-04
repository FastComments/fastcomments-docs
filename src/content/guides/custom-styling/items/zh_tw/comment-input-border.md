留言框周圍的邊框由文字區域自身的邊框加上小工具在其周圍繪製的幾條細線組成。若要更改其顏色或圓角，請設定這些 CSS 變數，而不是直接為 `textarea` 設定樣式。這些變數會一次重新設定邊框的每個部分，因此圓角和顏色始終保持一致。

| 變數 | 作用 | 預設值 |
|---|---|---|
| `--fc-input-border-color` | 邊框顏色 | `#bfbfbf` |
| `--fc-input-border-color-focus` | 使用者輸入時的邊框顏色 | `#555` |
| `--fc-input-border-radius` | 圓角的弧度 | `11px` |
| `--fc-input-border-start-start-radius` | 方形左上角的弧度（在從右到左的語言中為右上角） | `0` |

將 CSS 新增至 [Widget Customization 頁面](https://fastcomments.com/auth/my-account/customize-widget) 上的 **Custom CSS** 方塊，或透過 `customCSS` 選項傳遞。您只需設定想要更改的變數。

您也可以使用 Custom CSS 方塊正下方的 **Comment box border** 輔助工具，它會為您自動產生此 CSS。

## 更改邊框顏色

[inline-code-attrs-start title = '邊框顏色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## 更改輸入時的邊框顏色

[inline-code-attrs-start title = '輸入時的邊框顏色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## 讓四個角全部變圓

預設情況下左上角是方形的。將兩個圓角變數都設定為相同的值，即可讓四個角全部變圓：

[inline-code-attrs-start title = '讓四個角全部變圓'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## 方形邊角

[inline-code-attrs-start title = '方形邊角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## 符合您的品牌風格

[inline-code-attrs-start title = '品牌顏色與圓角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## 在深色模式中使用不同的顏色

當小工具處於深色模式時，它會帶有 `dark` class，因此您可以為深色模式設定不同的值：

[inline-code-attrs-start title = '深色模式的邊框顏色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## 為什麼不直接為文字區域設定樣式？

小工具會自行在文字區域周圍繪製留言框邊框的一部分。如果您只為 `textarea` 設定 `border-color` 或 `border-radius`，那些線條將保持預設樣式，邊框看起來會不一致，例如一條方形線條穿過圓角。上述變數會同時更改兩者。