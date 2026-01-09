---
FastComments 이메일 템플릿은 [EJS 템플릿 언어](https://github.com/mde/ejs/blob/main/docs/syntax.md)를 사용합니다.

변수를 출력하는 예시 문법은 `<%= object.someValue %>`이며, 조건문은 다음과 같이 작성할 수 있습니다:

    <% if (some_condition) { %>
        <div>Some content.</div>
    <% } else { %>
        <div>Some other content.</div>
    <% } %>

댓글 내용과 같은 원시 HTML을 출력하려면 다음과 같이 합니다: `<%- comment.commentHTML %>`. `=` 대신 `-`를 사용한다는 점에 주의하세요.

구문에 대한 자세한 문서는 위의 링크를 참조하세요.

---