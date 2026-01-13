[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments는 맞춤화되도록 설계되었습니다. 댓글 위젯 자체는 보안상 iframe 내부에서 실행되므로, 사용자 정의 스타일을 적용하려면
다음 두 가지 접근 방식 중 하나를 따라야 합니다.

The first, the easiest approach, and preferred by us, is to use the [widget customization page](https://fastcomments.com/auth/my-account/customize-widget).

In the widget customization page, see the "Show Advanced Options" section, under which there is an area labeled "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

이 접근 방식의 장점은 다음과 같습니다:
1. 입력한 CSS는 사용자에게 전송되기 전에 minify되며, 편집 UI에서 형식이 일관되게 유지됩니다.
2. 위젯 커스터마이즈 UI의 모든 이점을 누릴 수 있습니다. 예를 들어 사이트마다 댓글 위젯을 쉽게 다르게 커스터마이즈할 수 있습니다.
3. 저희가 댓글 위젯을 변경할 때, 귀하의 맞춤 스타일은 릴리스 프로세스의 일부로 테스트됩니다.

The second approach is to specify the **customCSS** parameter in the widget configuration, as follows:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

However, this has *limitations*:
1. 헤더 크기 때문에, 서버가 요청을 거부하기 전에 전달할 수 있는 사용자 정의 CSS의 양에는 한계가 있습니다.
2. 사용자 정의 CSS는 귀하의 인프라와 빌드 시스템에서 관리해야 합니다. 이는 단점이라기보다 장점일 수도 있습니다.
3. 이 경우 사용자 정의 CSS가 당사 서버로 전송된 후 다시 iframe 콘텐츠로 전송되므로 네트워크를 통해 **두 번** 전송되는 추가 오버헤드가 발생합니다. 다만 대부분의 페이로드 크기에서는 눈에 띄지 않습니다.
4. 일반적인 최적화는 네트워크 상의 크기를 줄이기 위해 CSS를 미니파이하는 것입니다. 그러나 이 접근 방식을 사용하는 경우 해당 작업을 직접 처리해야 합니다.
5. 저희가 변경을 할 때 귀하의 사용자 정의 CSS는 테스트되지 않습니다.

### External CSS Files

You can tell the widget to fetch an external file by using `@import`!

It's recommended to put the `@import` in a customization rule. This way, if we ever need to make a change to the comment widget, we can use our automation
tooling to verify your setup. So for example, you would create a customization rule in the Widget Customization UI, click `Advanced`, and enter in `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

You can also load an external CSS file via the `customCSS` property:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

However, remember that your CSS won't be able to be tested by us if you do this. 

### User Profile Modal Styling

User profile modals can also be styled with custom CSS. However, to ensure that custom styling is applied to user profiles, all CSS selectors must be prefixed with `.user-profile`. Without this prefix, custom styling will be ignored for user profile modals.

For example:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

At FastComments, we know our customers customize the commenting widget. That's by design - the last thing we want is for our product to cause design
inconsistencies in your product.

Since this is an important part of our product, we have a build pipeline that allows us to review changes to the comment widget, per-customer, each release.

If we find minor issues, we will update your account to ensure our release goes smoothly. If we see major breaking changes, this allows us to halt the release.

---