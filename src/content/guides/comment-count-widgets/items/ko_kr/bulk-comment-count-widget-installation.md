벌크 댓글 수 위젯은 동일한 페이지에서 여러 페이지의 댓글 수를 효율적으로 표시하도록 설계되었습니다. 각 댓글 수에 대해 개별 API 호출을 하는 대신, 이 위젯은 최적의 성능을 위해 요청을 일괄 처리합니다.

## 기본 설치

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## 작동 방식

벌크 위젯은 다음과 같이 작동합니다:

1. `fast-comments-count` 클래스를 가진 요소를 페이지에서 스캔
2. 각 요소에서 `data-fast-comments-url-id` 속성 읽기
3. 여러 댓글 수를 효율적으로 가져오기 위해 API 요청 일괄 처리
4. 각 요소를 적절한 댓글 수로 업데이트

## 구성 옵션

`FastCommentsCommentCountBulk` 함수는 다음 구성 옵션을 허용합니다:

- **tenantId** (필수): FastComments 테넌트 ID
- **apiHost** (선택사항): 자체 호스팅 인스턴스를 사용하는 경우 사용자 정의 API 호스트

## 실제 예제

블로그 게시물 목록에서 벌크 위젯을 사용하는 방법을 보여주는 실용적인 예제:

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## 성능 고려사항

벌크 위젯은 다음을 통해 자동으로 성능을 최적화합니다:

- **요청 일괄 처리**: 단일 API 호출로 여러 댓글 수를 가져옴
- **요청 크기 제한**: URL 목록이 너무 길어지면(1,000자 초과) 요청이 자동으로 분할됨
- **중복 제거**: 동일한 `data-fast-comments-url-id`를 가진 여러 요소가 동일한 수를 공유

## 동일한 URL ID를 가진 여러 요소

동일한 `data-fast-comments-url-id`를 가진 여러 요소를 페이지에 배치할 수 있습니다. 모두 동일한 수로 업데이트됩니다:

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## 로컬라이제이션

벌크 위젯은 FastComments 언어 설정에 따라 자동으로 댓글 수를 포맷합니다. 다음에 대한 적절한 텍스트를 제공합니다:

- 댓글 없음
- 댓글 1개
- 여러 댓글

## 벌크 vs 싱글 위젯 사용 시점

**벌크 위젯 사용 시:**
- 동일한 페이지에 여러 댓글 수가 있는 경우
- 댓글 수가 있는 게시물/기사 목록을 표시하는 경우
- 성능이 중요한 경우 (API 호출 감소)

**싱글 위젯 사용 시:**
- 페이지에 댓글 수가 하나만 필요한 경우
- 실시간 업데이트가 필요한 경우 (싱글 위젯은 실시간 업데이트 지원)
- 개별 위젯 동작을 더 세밀하게 제어하고 싶은 경우
