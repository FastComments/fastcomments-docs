최근 토론 위젯은 사이트에서 댓글 활동이 가장 최근인 페이지들을 보여줍니다. 각 항목에는 페이지 제목, 마지막 활동 날짜 및 총 댓글 수가 표시됩니다. 이 위젯은 어두운 배경을 자동으로 감지하여 스타일을 적절히 조정합니다.

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

## 구성 옵션

The `FastCommentsRecentDiscussionsV2` function accepts the following configuration options:

- **tenantId** (필수): 귀하의 FastComments 테넌트 ID
- **count** (선택사항): 표시할 페이지 수. 기본값은 `20`, 최대 `100`
- **hasDarkBackground** (선택사항): 다크 모드 스타일을 강제 적용합니다. 설정하지 않으면 페이지 배경에서 자동으로 감지됩니다

## 고급 예제

### 사용자 지정 개수

[inline-code-attrs-start title = '사용자 지정 개수의 최근 토론'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

[inline-code-attrs-start title = '다크 모드 적용된 최근 토론'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

---