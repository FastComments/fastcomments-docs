FastComments는 사용하기 쉬운 SSO 솔루션을 제공합니다. HMAC 기반 통합으로 사용자의 정보를 업데이트하는 것은 사용자가 업데이트된 페이로드로 페이지를 로드하도록 하는 것만큼 간단합니다.

그러나 응용 프로그램의 일관성을 높이기 위해 해당 흐름 외부에서 사용자를 관리하는 것이 바람직할 수 있습니다.

SSO User API는 우리가 SSOUsers라고 부르는 객체를 CRUD할 수 있는 방법을 제공합니다. 이 객체들은 일반 Users와 다르며 타입 안전성을 위해 분리되어 관리됩니다.

SSOUser 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'SSOUser 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // 관리자 권한 - 이 플래그가 설정된 SSO 사용자는 SSO 관리자으로 과금됩니다 (일반 SSO 사용자와 별도)
    isAdminAdmin?: boolean // 관리자 권한 - 이 플래그가 설정된 SSO 사용자는 SSO 관리자으로 과금됩니다 (일반 SSO 사용자와 별도)
    isCommentModeratorAdmin?: boolean // 중재자 권한 - 이 플래그가 설정된 SSO 사용자는 SSO 중재자로 과금됩니다 (일반 SSO 사용자와 별도)
    /** null이면 액세스 제어가 사용자에게 적용되지 않습니다. 빈 리스트이면 이 사용자는 어떤 페이지도 볼 수 없고 다른 사용자를 @mention할 수도 없습니다. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** 다른 사용자가 이 사용자의 프로필에서 댓글을 포함한 활동을 보지 못하게 합니다. 기본값은 안전한 프로필을 제공하기 위해 true입니다. **/
    isProfileActivityPrivate?: boolean
    /** 다른 사용자가 이 사용자의 프로필에 댓글을 남기거나 기존 프로필 댓글을 보는 것을 허용하지 않습니다. 기본값은 false입니다. **/
    isProfileCommentsPrivate?: boolean
    /** 다른 사용자가 이 사용자에게 직접 메시지를 보내는 것을 허용하지 않습니다. 기본값은 false입니다. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** 사용자 배지에 대한 선택적 구성. **/
    badgeConfig?: {
        /** 사용자에게 할당할 배지 ID의 배열. 최대 30개의 배지로 제한됩니다. 순서가 유지됩니다. **/
        badgeIds: string[]
        /** true이면 표시된 모든 기존 배지를 제공된 배지로 대체합니다. false이면 기존 배지에 추가합니다. **/
        override?: boolean
        /** true이면 테넌트 구성에서 배지 표시 속성을 업데이트합니다. **/
        update?: boolean
    }
}
[inline-code-end]

### SSO 사용자 청구

SSO 사용자는 권한 플래그에 따라 다르게 과금됩니다:

- **Regular SSO Users**: 관리자 또는 중재자 권한이 없는 사용자는 일반 SSO 사용자로 과금됩니다
- **SSO Admins**: `isAccountOwner` 또는 `isAdminAdmin` 플래그가 있는 사용자는 SSO 관리자로 별도 과금됩니다 (일반 테넌트 관리자와 동일한 요율)
- **SSO Moderators**: `isCommentModeratorAdmin` 플래그가 있는 사용자는 SSO 중재자로 별도 과금됩니다 (일반 중재자와 동일한 요율)

**중요**: 이중 과금을 방지하기 위해 시스템은 이메일 주소를 기준으로 SSO 사용자와 일반 테넌트 사용자 및 중재자를 자동으로 중복 제거합니다. SSO 사용자가 일반 테넌트 사용자나 중재자와 동일한 이메일을 가지고 있으면 두 번 과금되지 않습니다.

### 액세스 제어

사용자는 그룹으로 나눌 수 있습니다. 이것이 `groupIds` 필드의 목적이며 선택적입니다.

### @멘션

기본적으로 `@mentions`는 `@` 문자를 입력하면 다른 sso 사용자를 검색하기 위해 `username`을 사용합니다. `displayName`이 사용되면 `displayName`에 일치하는 결과가 있을 때 `username`과 일치하는 결과는 무시되며 `@mention` 검색 결과는 `displayName`을 사용합니다.

### 구독

FastComments에서는 사용자가 댓글 위젯의 벨 아이콘을 클릭하고 구독을 클릭하면 페이지를 구독할 수 있습니다.

일반 사용자와 함께 우리는 알림 설정에 따라 알림 이메일을 전송합니다.

SSO 사용자와 함께, 하위 호환성을 위해 이를 분리합니다. 사용자는 `optedInSubscriptionNotifications`를 `true`로 설정한 경우에만 이러한 추가 구독 알림 이메일을 받게 됩니다.

### 배지

`badgeConfig` 속성을 사용하여 SSO 사용자에게 배지를 할당할 수 있습니다. 배지는 댓글에서 사용자 이름 옆에 표시되는 시각적 표시기입니다.

- `badgeIds` - 사용자에게 할당할 배지 ID의 배열입니다. 이는 FastComments 계정에서 생성된 유효한 배지 ID여야 합니다. 30개의 배지로 제한됩니다.
- `override` - true이면 댓글에 표시된 모든 기존 배지가 제공된 배지로 대체됩니다. false이거나 생략된 경우 제공된 배지가 기존 배지에 추가됩니다.
- `update` - true이면 사용자가 로그인할 때마다 테넌트 구성에서 배지 표시 속성이 업데이트됩니다.