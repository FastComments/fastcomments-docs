[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

When rendering a comment thread, or leaving a comment, FastComments needs to know what page, or article, or product
those comments belong to.

댓글 스레드를 렌더링하거나 댓글을 남길 때, FastComments는 해당 댓글이 어느 페이지, 기사, 혹은 제품에 속하는지 알아야 합니다.

To do this, we use something we call the "URL ID". It's either an identifier, like a string or a number, or a URL.

이를 위해 우리는 "URL ID"라고 부르는 것을 사용합니다. 이는 문자열이나 숫자와 같은 식별자이거나 URL일 수 있습니다.

By default, if you do not specify the urlId, it will become the page URL. We will take the current page URL, and clean it to remove
any common marketing parameters or tracking identifiers.

기본적으로 urlId를 지정하지 않으면 페이지 URL이 사용됩니다. 현재 페이지 URL을 가져와 일반적인 마케팅 파라미터나 추적 식별자를 제거하도록 정리합니다.

In the case of third party integrations, like WordPress, our plugin will usually use the identifier that represents the current information being viewed as
the URL ID, for example the article/page id.

WordPress와 같은 서드파티 통합의 경우, 플러그인은 일반적으로 현재 보고 있는 정보(예: 기사/페이지 ID)를 URL ID로 사용합니다.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = '사용자 정의 URL ID 정의'; code-example-end]

One thing that we'll often reference in this document is the <a href="https://fastcomments.com/auth/my-account/customize-widget/new">위젯 커스터마이징 UI</a>.

This UI can be used to make many changes to the comment widget without using code.

When creating a customization rule, we'll often want it to apply to all pages to our site. However, in some cases we want to customize the comment widget
on a particular page, either to apply custom styling, or maybe make comments for that particular page anonymous. You could also, for example, have live comments
appear right away on some pages, while hiding them under notification buttons on others.

This is all possible via the URL ID input field on this page, which looks like as follows:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='커스터마이징 규칙을 한 페이지에 적용하거나 */blog/*와 같은 패턴에 적용하기 위해 사용되는 URL ID 필드'; title='위젯 커스터마이징 페이지의 URL ID 입력' app-screenshot-end]

The value in this field should match the *urlId* parameter passed into the comment widget. If you want your customization rule to be *urlId* agnostic, leave this field empty or enter *.

이 필드의 값은 댓글 위젯에 전달되는 *urlId* 매개변수와 일치해야 합니다. 커스터마이징 규칙을 *urlId*와 무관하게 만들고 싶다면, 이 필드를 비워두거나 *를 입력하세요.

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

2023년 현재 위젯 커스터마이징의 `URL ID` 필드는 이제 패턴도 지원합니다! 예를 들어 `*/blog/*`를 사용해 블로그에 특화된 스타일을 적용하고, `*/store/*`를 사용해 스토어에 특화된 스타일을 적용할 수 있으며, 모두 동일한 도메인을 사용합니다.

### 주의사항

1. If your page has hash parameters (like example.com#page-1) - this will become part of the URL ID, by default.
1. 페이지에 해시 파라미터가 있는 경우(예: example.com#page-1) - 기본적으로 이것이 URL ID의 일부가 됩니다.
2. During migrations, for example from WordPress to Gatsby, you may have to migrate the URL ID comment values after the initial migration. For that, reach out to us.
2. 마이그레이션 중, 예를 들어 WordPress에서 Gatsby로 이동할 때 초기 마이그레이션 후 URL ID 댓글 값을 마이그레이션해야 할 수 있습니다. 이를 위해서는 저희에게 연락해 주세요.

---