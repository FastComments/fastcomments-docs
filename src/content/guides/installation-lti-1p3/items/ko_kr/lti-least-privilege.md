FastComments LTI 1.3 통합은 최소 권한 원칙을 따릅니다: 사용자 식별, 댓글을 올바른 코스와 리소스에 연결, 역할 기반 권한 적용에 필요한 런치 클레임만 사용합니다.

나머지 내용은 통합이 사용하는 모든 클레임, 요청하지 않는 모든 LTI Advantage 서비스, 수집하지 않는 모든 데이터 범주를 매핑합니다. 보안 및 조달 검토자는 아래 표에서 직접 답변을 발췌할 수 있습니다.

## LMS로부터 수신되는 데이터 요소

모든 LTI 1.3 런치는 LMS에서 서명된 JWT를 전달합니다. FastComments는 해당 JWT에서 다음 클레임을 추출하며 그 외의 것은 사용하지 않습니다:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | 런치 간에 사용자를 일관되게 식별하여 동일한 사용자가 동일한 FastComments SSO 사용자로 해석되도록 함 | 예 | 예, 안정적인 내부 SSO ID의 일부로 저장 |
| Display name | `name` | 사용자의 댓글 옆에 표시되는 표기명 | 예 (부재 시 "LMS 사용자"로 대체) | 예 |
| Email | `email` | 계정 매칭, 알림, 중재, 지원 연락 | 선택사항 (이메일 없이도 통합 작동) | 제공된 경우 예 |
| Avatar URL | `picture` | 사용자의 댓글에 표시되는 이미지 | 선택사항 | URL만 저장; FastComments는 이미지를 다운로드하거나 재호스팅하지 않음 |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | 사용자가 관리자, 강사(중재자), 학습자인지 결정 | 예 | SSO 세션에서 파생된 `isAdmin` / `isModerator` 플래그 |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | 댓글 스레드를 올바른 LMS 코스에 연결 | 예 | 예, 해결된 페이지 식별자의 일부로 저장 |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | 코스 내의 올바른 활동 또는 도구 배치와 댓글을 연관시킴 | 존재하는 경우 예 | 예, 해결된 페이지 식별자의 일부로 저장 |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | 런치를 올바른 FastComments 테넌트 구성으로 라우팅 | 예 | 예, FastComments LTI 구성 레코드에 저장 |

## 등록 시 선언된 클레임 및 범위

LTI 1.3 동적 등록 중에, FastComments는 `scope: ""` (추가 OAuth 범위 없음)로 자체를 등록하고 다음 OpenID Connect 클레임만 선언합니다:

`iss`, `sub`, `name`, `email`, `picture`

두 가지 메시지 유형을 등록합니다:

- LtiResourceLinkRequest - FastComments로의 표준 코스 런치입니다.
- LtiDeepLinkingRequest - 강사가 코스 내에 FastComments 도구를 배치할 수 있게 합니다.

LMS로부터 추가 액세스 토큰은 요청하지 않습니다.

## 요청하지 않는 LTI Advantage 서비스

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | 아니요 | 통합은 코스 로스터를 필요로 하지 않음; 사용자 신원은 각 런치에 포함되어 도착함 |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | 아니요 | 통합은 성적부(gradebook)를 인식하지 않음 |
| Deep Linking beyond the standard placement return | 추가 데이터 없음 | 딥링킹은 강사가 도구를 배치할 때만 사용됨; 코스 콘텐츠를 나열하지 않음 |

## 수집하지 않는 데이터

LTI 자체를 제외하고, FastComments는 LMS나 사용자로부터 다음을 요청하거나 수신하지 않습니다:

| Category | Collected? |
|----------|------------|
| Student grades | 아니요 |
| Assignment submissions | 아니요 |
| Attendance records | 아니요 |
| Full course rosters | 아니요 |
| Government identifiers | 아니요 |
| Date of birth | 아니요 |
| Postal address or phone number | 아니요 |
| Financial information | 아니요 |
| LMS administrator credentials | 아니요 |

## 접근 경계

- FastComments는 LMS의 등록된 키로 서명된 권한 있는 LTI 1.3 런치 내부의 데이터만 수신합니다. 통합은 추가 정보를 위해 LMS로 콜백 호출을 하지 않습니다.
- 런치 토큰은 일회용이며 수명이 짧습니다. 재생된 토큰이나 만료된 토큰은 거부됩니다.
- LMS 관리자는 도구가 플랫폼 내 어디에 배포되는지를 제어합니다. 예를 들어 D2L Brightspace는 배포별 조직 단위 범위 지정 및 배포별 보안 설정을 지원하여 관리자가 도구를 전체적으로 사용 가능하게 하는 대신 특정 코스나 조직 단위로 제한할 수 있게 합니다. Moodle, Blackboard, Sakai 및 Schoology도 그들의 LTI 1.3 구현에서 동등한 배포별 제어를 제공합니다.

## 저장 및 보존

FastComments는 활성 댓글 서비스 기간 동안 및 고객이 구성한 보존 설정에 따라 LTI에서 파생된 데이터를 보관합니다. 댓글 데이터는 저장 시 암호화된 상태의 프로덕션 스토리지에 저장됩니다. 계정 종료 또는 서면 삭제 요청 시, FastComments는 해당 계약에 따라 고객 데이터를 삭제하거나 익명화합니다.

저장 및 데이터 처리에 대한 전체 세부 정보는 <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments 개인정보 처리방침</a>을 참조하십시오.

## 검토 주기

추가 클레임, 범위 또는 LTI Advantage 서비스가 필요한 새로운 LTI 기능은 출시 전에 요청된 접근이 기능에 비례하고 필요한지 확인하기 위해 검토됩니다.

## 보안 설문용 간단 문구

> FastComments는 LTI 1.3 통합에 최소 권한 및 데이터 최소화를 적용합니다. 통합은 사용자 인증에 필요한 LTI 런치 클레임(`sub`, `name`, `email`, `picture`), 사용자의 역할을 판단하는 정보, 그리고 댓글이 속한 코스와 리소스를 식별하는 정보만 사용합니다. FastComments는 Names and Role Provisioning Services, Assignment and Grade Services, 성적부 데이터, 출석, 전체 로스터 또는 LMS 관리자 접근을 요청하지 않습니다. LMS 관리자는 도구가 사용 가능한 조직 단위, 코스 및 배포를 계속 제어합니다.