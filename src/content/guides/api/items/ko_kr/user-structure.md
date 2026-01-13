`User`는 모든 사용자들의 공통 분모를 나타내는 객체입니다.

FastComments에서는 사용자에 대한 여러 가지 다른 사용 사례가 있다는 점을 염두에 두세요:

- Secure SSO
- Simple SSO
- Tenant Users (For example: Administrators)
- Commenters

이 API는 **Commenters** 및 **Simple SSO**를 통해 생성된 사용자들을 위한 것입니다. 기본적으로 사이트를 통해 생성된 모든 사용자는 이 API로 접근할 수 있습니다. Tenant Users도 이렇게 가져올 수 있지만, `/tenant-users/` API와 상호작용하면 더 많은 정보를 얻을 수 있습니다.

`Secure SSO`의 경우에는 `/sso-users/` API를 사용하세요.

이 유형의 사용자는 업데이트할 수 없습니다. 이들은 귀하의 사이트를 통해 계정을 생성했기 때문에 일부 읽기 전용 기본 액세스만 제공되며 변경은 할 수 없습니다. 이런 흐름을 원하시면 `Secure SSO`를 설정해야 합니다.

`User` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'User 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** 이것은 댓글 객체의 userId로도 사용되는 id입니다. **/
    id: string
    username: string
    /** 예: 댓글 작성자의 블로그 링크입니다. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]