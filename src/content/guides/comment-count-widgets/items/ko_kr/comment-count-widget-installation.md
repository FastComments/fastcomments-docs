댓글 수 위젯은 단일 페이지의 댓글 수를 표시하도록 설계되었습니다. 가볍고 구성하면 실시간 업데이트를 제공합니다.

## 기본 설치

[inline-code-attrs-start title = 'Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## 구성 옵션

`FastCommentsCommentCount` 함수는 다음 구성 옵션을 허용합니다:

- **tenantId** (필수): FastComments 테넌트 ID
- **urlId** (선택사항): 페이지 식별자. 지정하지 않으면 `window.location.href`가 기본값
- **numberOnly** (선택사항): `true`이면 텍스트 없이 숫자만 표시. 기본값은 `false`
- **isLive** (선택사항): `true`이면 카운트가 자동으로 업데이트됨. 기본값은 `false`

## 고급 예제

### 사용자 정의 URL ID

[inline-code-attrs-start title = 'Comment Count with Custom URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-custom"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-custom'), {
        tenantId: 'demo',
        urlId: 'my-custom-page-id'
    });
</script>
[inline-code-end]

### 숫자만 표시

[inline-code-attrs-start title = 'Comment Count Number Only'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-number"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-number'), {
        tenantId: 'demo',
        numberOnly: true
    });
</script>
[inline-code-end]

### 실시간 업데이트

[inline-code-attrs-start title = 'Live Comment Count Updates'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-live"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-live'), {
        tenantId: 'demo',
        isLive: true
    });
</script>
[inline-code-end]

## 위젯 메서드

위젯은 다음 메서드를 가진 객체를 반환합니다:

- **destroy()**: 위젯을 제거하고 모든 타이머를 정리
- **update(config)**: 새 구성으로 위젯을 업데이트

### 사용 예제

[inline-code-attrs-start title = 'Widget Methods Example'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-methods"></div>
<script>
    const widget = window.FastCommentsCommentCount(document.getElementById('comment-count-methods'), {
        tenantId: 'demo'
    });

    // Update the widget to show a different page's count
    setTimeout(() => {
        widget.update({
            tenantId: 'demo',
            urlId: 'different-page-id'
        });
    }, 5000);

    // Destroy the widget after 10 seconds
    setTimeout(() => {
        widget.destroy();
    }, 10000);
</script>
[inline-code-end]

## 스타일링

위젯은 댓글 수가 포함된 일반 HTML을 렌더링하며 최소한의 스타일링이 제공됩니다. CSS로 외관을 사용자 정의할 수 있습니다:

[inline-code-attrs-start title = 'Custom Styling'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<style>
    .comment-count-styled {
        background: #f0f0f0;
        padding: 5px 10px;
        border-radius: 15px;
        font-size: 14px;
        color: #666;
        display: inline-block;
    }
</style>
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-styled" class="comment-count-styled"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-styled'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]
