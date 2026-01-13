가장 작은 확장 프로그램은 다음과 같습니다:

[inline-code-attrs-start title = '간단한 확장'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

이 예시를 위해, 이것을 `my-extension.js`로 저장하고 `https://example.com/my-extension.min.js`에서 접근할 수 있게 하세요.

이 확장 프로그램은 아무 작업도 하지 않습니다. 다만 로드될 때 코어 댓글 라이브러리가 생성한 확장 객체를 가져옵니다.

이 `Extension` 객체는 싱글톤이며 다른 확장과 공유되지 않습니다.

다음으로, 확장을 로드하려면 댓글 위젯에 이를 알려야 합니다. 예를 들어:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

실용적인 예제는 다음 섹션을 참조하세요.

---