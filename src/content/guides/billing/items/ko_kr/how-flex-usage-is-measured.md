Flex 인보이스는 사용 종류별로 한 줄씩 구성된 항목 목록으로, 각 항목에는 수량과 요율이 표시됩니다. 예를 들어 `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`와 같습니다. 요율은 [가격 페이지](https://fastcomments.com/traffic-pricing)와 [청구 정보](https://fastcomments.com/auth/my-account/billing-info) 페이지에서 확인할 수 있습니다. 이 섹션에서는 각 수량의 의미를 설명합니다.

### 월 단위로 집계

매월 초기화되며, 달력상 월의 첫날부터 마지막 날까지(UTC) 집계됩니다:

- **Page Loads**: 페이지에서 댓글 위젯이나 피드가 로드된 횟수입니다. 표시되는 댓글 수와 관계없이 댓글 위젯이 로드될 때마다 집계됩니다. 댓글 스레드별 또는 페이지별 청구는 없습니다.
- **API Credits**: API 키를 사용한 REST API 호출입니다.
- **Small Widgets Credits**: 댓글 수 및 배지와 같은 소형 위젯의 로드 횟수입니다.
- **LLM Tokens**: [AI Agents](/guide-ai-agents.html) 및 기타 LLM 기능에서 사용된 토큰입니다.

### 월말에 집계

인보이스가 생성될 때 찍힌 스냅샷입니다:

- **Comments Stored**: 이번 달에 추가된 댓글이 아니라 계정의 전체 댓글 수입니다. 댓글을 삭제하면 이 수치가 줄어들지만, 답글이 달린 삭제된 댓글은 답글의 맥락이 유지되도록 플레이스홀더로 보관됩니다.
- **SSO Users**: 사이트에서 생성한 SSO 사용자의 총 수로, 오래전에 한 번 댓글을 단 사람도 포함됩니다. 이 항목이 청구액에서 가장 큰 비중을 차지한다면, 해당 월에 로그인한 사용자만 집계되는 월간 활성 사용자(MAU) 기준 청구에 대해 지원팀에 문의하세요.
- **Moderators**, **Admins**, **SSO Moderators**, **SSO Admins**: 좌석 수로, [Admin and Moderator Seats](#admin-and-moderator-seats)에서 다룹니다. 첫 번째 관리자는 무료입니다.
- **Domains**: 계정에 등록된 사이트입니다. 첫 번째 사이트는 무료이며, 추가 사이트는 계정에 있는 동안 매월 청구됩니다. 서브도메인은 별도의 사이트로 추가한 경우에만 집계되며, 하나의 최상위 도메인이 해당 서브도메인을 포괄합니다. [Multiple Sites](/guide-multiple-sites.html#domain-limits)를 참조하세요.
- **Managed Tenants**: 이 계정이 관리하는 하위 계정이 있다면 해당 계정들과 그 사용량이 별도 항목으로 추가됩니다.

### 최소 금액

항목 합계가 Flex 최소 금액보다 적으면 인보이스는 최소 금액으로 보정됩니다. 관리자 한 명과 도메인 하나를 가진 소규모 사이트는 최소 금액만 지불합니다.

### 인보이스 미리보기

[청구 분석](https://fastcomments.com/auth/my-account/analytics/billing) 페이지에서는 다음 인보이스의 모든 항목을 오늘 기준 수치로 보여주므로 놀랄 일이 없습니다. 페이지 로드, API, 소형 위젯 집계는 최대 1분까지 지연될 수 있으며, 나머지는 실시간입니다.

### 기타 통화

Flex 사용량은 미국 달러로 책정됩니다. 인보이스 통화가 다른 경우 각 항목은 인보이스 생성일의 환율로 변환되므로, 동일한 사용량이라도 월마다 금액이 약간 달라질 수 있습니다. [Currencies](#currencies)를 참조하세요.