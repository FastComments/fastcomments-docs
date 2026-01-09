熱門頁面小工具會顯示留言數最多的頁面清單。

此小工具包含最低限度的預設樣式，並設計為可輕易使用您自己的 CSS 進行自訂。

## 小工具結構

該小工具會呈現以下的 HTML 結構：

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## 熱門頁面預設 CSS 參考

該小工具包含下列最低限度的預設樣式：

[inline-code-attrs-start title = '熱門頁面小工具預設 CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## 自訂範例

### 為連結新增樣式
```css
.fastcomments-top-pages .title-link {
    color: #0066cc !important;
    text-decoration: none !important;
    font-size: 14px !important;
}

.fastcomments-top-pages .title-link:hover {
    text-decoration: underline !important;
}
```

### 在頁面間加入邊框
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### 樣式化留言數
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---