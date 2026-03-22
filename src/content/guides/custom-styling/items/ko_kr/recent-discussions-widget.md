최근 토론 위젯은 최신 댓글 활동에 따라 정렬된 페이지 목록을 표시합니다. 헤딩, 마지막 활동 날짜, 아이콘이 포함된 댓글 수, 자동 다크 모드 감지를 포함합니다.

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

- **tenantId** (required): Your FastComments tenant ID
- **count** (optional): 표시할 페이지 수. 기본값은 `20`, 최대 `100`
- **hasDarkBackground** (optional): 다크 모드 스타일을 강제 적용합니다. 설정하지 않으면 페이지 배경에서 자동으로 감지됩니다

## 위젯 구조

위젯은 다음 HTML 구조로 렌더링됩니다:

[inline-code-attrs-start title = '최근 토론 위젯 HTML 구조'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rd2">
    <div class="fc-rd2-heading">Recent Discussions</div>
    <div class="fc-rd2-list">
        <div class="fc-rd2-item">
            <div class="fc-rd2-detail">
                <a class="fc-rd2-title" href="...">Page Title</a>
                <span class="fc-rd2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-rd2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## 기본 CSS 참조

[inline-code-attrs-start title = '최근 토론 위젯 기본 CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rd2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rd2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rd2-item:last-child { border-bottom: none; }
.fc-rd2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-rd2-activity { font-size: 11px; color: #999; }
.fc-rd2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## 사용자 정의 예제

### 컨테이너 테두리 제거

[inline-code-attrs-start title = '컨테이너 테두리 제거'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

### 맞춤 링크 색상

[inline-code-attrs-start title = '맞춤 링크 색상'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
a.fc-rd2-title:hover {
    color: #e63946 !important;
}
[inline-code-end]

---