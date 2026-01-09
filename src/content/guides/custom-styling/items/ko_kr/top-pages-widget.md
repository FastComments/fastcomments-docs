상위 페이지 위젯은 댓글이 가장 많은 페이지 목록을 표시합니다.

이 위젯은 최소한의 기본 스타일링을 포함하며 사용자가 직접 CSS로 쉽게 커스터마이즈할 수 있도록 설계되어 있습니다.

## 위젯 구조

위젯은 다음과 같은 HTML 구조로 렌더링됩니다:

```html
<div class="fastcomments-top-pages">
    <div class="page">
        <a class="title-link" href="...">Page Title (42)</a>
    </div>
    <!-- More pages... -->
</div>
```

## 상위 페이지 기본 CSS 참고

이 위젯은 다음과 같은 최소한의 기본 스타일을 포함합니다:

[inline-code-attrs-start title = '상위 페이지 위젯 기본 CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-top-pages {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
}
.fastcomments-top-pages .page {
    padding-top: 5px;
}
[inline-code-end]

## 사용자 지정 예제

### 링크에 스타일 추가
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

### 페이지 사이에 테두리 추가
```css
.fastcomments-top-pages .page {
    border-bottom: 1px solid #eee !important;
    padding: 10px 0 !important;
}

.fastcomments-top-pages .page:last-child {
    border-bottom: none !important;
}
```

### 댓글 수 스타일 지정
```css
.fastcomments-top-pages .title-link {
    display: flex !important;
    justify-content: space-between !important;
}
```

---