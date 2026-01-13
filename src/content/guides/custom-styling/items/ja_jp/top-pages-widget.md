Top Pagesウィジェットは、コメント数が最も多いページの一覧を表示します。

このウィジェットには最小限のデフォルトスタイルが含まれており、独自のCSSで簡単にカスタマイズできるように設計されています。

## ウィジェットの構造

ウィジェットは次のHTML構造でレンダリングされます:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## Top Pages のデフォルトCSS参照

ウィジェットには次の最小限のデフォルトスタイルが含まれます:

[inline-code-attrs-start title = 'Top Pages ウィジェットのデフォルト CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## カスタマイズ例

### リンクにスタイルを追加
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

### ページ間にボーダーを追加
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### コメント数のスタイル
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---