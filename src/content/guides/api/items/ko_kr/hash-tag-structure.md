`HashTag` 객체는 사용자가 남길 수 있는 태그를 나타냅니다. HashTags는 외부 콘텐츠로 연결하거나
관련된 댓글들을 묶는 데 사용할 수 있습니다.

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'HashTag 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** "#" 또는 원하는 문자로 시작해야 합니다. **/
    tag: string
    /** 해시태그가 가리킬 수 있는 선택적 URL입니다. 해시태그로 댓글을 필터링하는 대신, UI는 클릭 시 이 URL로 리디렉션합니다. **/
    url?: string
    /** 읽기 전용 **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.