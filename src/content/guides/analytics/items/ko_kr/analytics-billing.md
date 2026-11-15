[청구 분석 페이지](https://fastcomments.com/auth/my-account/analytics/billing)에서 요금이 부과되거나 사용량이 제한될 수 있는 모든 지표가 보고됩니다.

추적되는 지표의 예는 다음과 같습니다:

- 페이지 로드 수 (현재 월 기준)
- API 크레딧 (현재 월 기준)
- 저장된 신규 댓글 수 (현재 월 기준)
- 테넌트 사용자
- SSO 사용자 (관리자/중재자 권한이 없는 일반 SSO 사용자)
- SSO 관리자 (`isAccountOwner` 또는 `isAdminAdmin` 플래그가 있는 SSO 사용자 - 일반 SSO 사용자와 별도로 청구됨)
- SSO 중재자 (`isCommentModeratorAdmin` 플래그가 있는 SSO 사용자 - 일반 SSO 사용자와 별도로 청구됨)
- 중재자
- 도메인

`Creators` 및 `Pro` 플랜의 경우, 이러한 지표는 계정 한도에 근접했거나 도달했는지 확인하기 위한 참고 자료로만 사용됩니다.

`Flex` 플랜의 경우, 이 페이지는 **다음** 청구서에 대한 현재 사용량의 비용 내역을 보여줍니다. 이를 통해 청구가 이루어지기 전에 어떻게 청구될지 쉽게 파악할 수 있습니다. 과거 청구서는 [청구 내역](https://fastcomments.com/auth/my-account/billing-history)에서 확인할 수 있습니다. 각 항목이 계산되는 방식과 청구서가 청구되는 시점은 [청구 가이드](/guide-billing.html#how-flex-usage-is-measured)에 설명되어 있습니다.

`페이지 로드 수`, `API 크레딧`, `저장된 신규 댓글 수` 지표는 최대 1분까지 지연될 수 있습니다. 다른 지표는 실시간입니다.

이 정보는 기록 보관 목적으로 월 단위로 1년 동안, 또는 테넌트를 삭제할 때까지 저장되지만, UI에는 현재 기간만 표시됩니다.