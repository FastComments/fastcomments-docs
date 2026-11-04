`HashTag` 객체는 사용자가 남길 수 있는 태그를 나타냅니다. HashTag는 외부 콘텐츠로 연결하거나 관련 댓글을 묶는 데 사용할 수 있습니다.

`HashTag` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'HashTag 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** "#" 또는 원하는 문자로 시작해야 합니다. **/
    tag: string
    /** 해시태그가 가리킬 수 있는 선택적 URL입니다. 해시태그로 댓글을 필터링하는 대신, 클릭 시 UI가 이 URL로 리디렉션합니다. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

참고 사항:

- 일부 API 엔드포인트에서는 해시태그가 URL에 사용되는 것을 볼 수 있습니다. 값은 URI 인코딩해야 함을 기억하세요. 예를 들어, `#`은 `%23`으로 표현해야 합니다.
- 일부 필드는 `READONLY`로 표시되어 있습니다 - 이는 API에서 반환되지만 설정할 수는 없습니다.