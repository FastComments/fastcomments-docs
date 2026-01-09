### 기본 예제

Image Chat를 사용하는 가장 간단한 방법은 단일 이미지 요소를 대상으로 하는 것입니다. 이 예제는 이미지에서 인터랙티브한 토론을 활성화하는 방법을 보여줍니다:

[inline-code-attrs-start title = '기본 이미지 채팅 예제'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### 컨테이너 요소 예제

이미지가 포함된 컨테이너 요소를 전달할 수도 있습니다:

[inline-code-attrs-start title = '컨테이너 포함 이미지 채팅'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### 사용자 정의 URL ID 예제

기본적으로 Image Chat은 페이지 URL과 이미지 소스 및 좌표를 결합하여 대화를 식별합니다. 사용자 정의 `urlId`를 제공할 수 있습니다:

[inline-code-attrs-start title = '사용자 정의 URL ID 이미지 채팅'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

URL 구조가 변경되더라도 동일한 대화를 유지하거나 여러 페이지에서 동일한 토론 포인트를 공유하려는 경우 유용합니다.

### 다크 모드 예제

사이트에 어두운 배경이 있고 위젯이 자동으로 이를 감지하지 못하는 경우, 수동으로 다크 모드 지원을 활성화할 수 있습니다:

[inline-code-attrs-start title = '다크 모드 이미지 채팅'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### 사용자 지정 채팅 사각형 크기 예제

이미지에 표시되는 클릭 가능한 사각형의 크기를 조정할 수 있습니다. 크기는 이미지 너비의 백분율로 지정됩니다:

[inline-code-attrs-start title = '사용자 지정 사각형 크기 이미지 채팅'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // 더 작은 사각형 (기본값은 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### 댓글 수 콜백 예제

`commentCountUpdated` 콜백을 사용하여 댓글이 추가되거나 업데이트될 때를 추적합니다:

[inline-code-attrs-start title = '댓글 수 콜백 이미지 채팅'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### 여러 이미지 예제

여러 이미지에 대해 Image Chat을 초기화할 수 있습니다. 각 이미지는 자체 독립된 토론 지점을 갖습니다:

[inline-code-attrs-start title = '여러 이미지에서 이미지 채팅'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // 첫 번째 이미지에서 초기화
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // 두 번째 이미지에서 초기화
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---