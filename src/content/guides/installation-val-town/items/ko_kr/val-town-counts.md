---
인덱스 페이지에서는 행마다 하나의 comment-count 위젯을 렌더링하지 마세요. 이는 게시물당 하나의 요청이 발생합니다. 전체 페이지에 대해 단일 요청으로 처리되는 bulk count를 사용하세요.

각 행에 해당 스레드가 사용하는 `urlId`를 표시하고, bulk 위젯을 한 번만 로드하세요:

[inline-code-attrs-start title = '인덱스 페이지의 대량 댓글 수'; type='javascript' inline-code-attrs-end]
[inline-code-start]
<ul>
  {posts.map((post) => (
    <li>
      <a href={post.slug}>{post.title}</a>{" "}
      <span class="fast-comments-count" data-fast-comments-url-id={post.slug}></span>
    </li>
  ))}
</ul>

<script
  dangerouslySetInnerHTML={{
    __html: `window.FastCommentsBulkCountConfig = ${
      JSON.stringify({ tenantId: TENANT_ID })
    };`,
  }}
/>
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>
[inline-code-end]

스크립트는 페이지 내 모든 `.fast-comments-count` 요소를 찾아 해당 카운트를 채워 넣습니다.

`data-fast-comments-url-id`는 게시물의 댓글 위젯이 사용하는 `urlId`와 일치해야 합니다. 위젯이 슬러그를 사용한다면, 마커도 슬러그를 사용합니다. 일치하지 않을 경우 댓글이 있는 스레드에서도 0이 표시됩니다.

스크립트는 `window.FastCommentsBulkCountConfig`를 폴링하므로, 스크립트 태그 앞이나 뒤에 설정을 넣는 것이 중요하지 않습니다.
---