최근 토론 위젯은 사이트에서 가장 최근에 댓글 활동이 있었던 페이지들을 보여줍니다. 각 항목에는 페이지 제목, 마지막 활동 날짜, 총 댓글 수가 표시됩니다. 어두운 배경을 자동으로 감지하여 그에 맞게 스타일을 조정합니다.

## 기본 설치

[inline-code-attrs-start title = '최근 토론 위젯 설치'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## 설정 옵션

`FastCommentsRecentDiscussionsV2` 함수는 다음 설정 옵션을 받습니다:

- **tenantId** (필수): FastComments 테넌트 ID
- **count** (선택): 표시할 페이지 수. 기본값은 `20`, 최대 `100`
- **hasDarkBackground** (선택): 다크 모드 스타일 강제 적용. 설정하지 않으면 페이지 배경에서 자동 감지됩니다

## 고급 예제

### 사용자 지정 개수

[inline-code-attrs-start title = '사용자 지정 개수를 사용하는 최근 토론'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### 다크 모드 강제 적용

[inline-code-attrs-start title = '다크 모드를 사용하는 최근 토론'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]