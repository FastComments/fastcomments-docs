`tenantId: "demo"`은 공유 퍼블릭 샌드박스입니다. 회원가입 없이도 작동하므로 예제에서 이를 사용하지만, FastComments를 시도하는 다른 모든 사용자는 동일한 스레드에 글을 쓰고 누구나 이를 관리할 수 있습니다. 중요한 내용을 게시하기 전에 전환하세요.

Your tenant ID is on the [API secret page](https://fastcomments.com/auth/my-account/api-secret).

A tenant ID is public and belongs in browser code. An API secret does not, and nothing on this page needs one.

## 환경 변수에서 읽기

Val Town 값은 무료 티어에서 공개되어 있기 때문에 소스가 전 세계에서 읽을 수 있습니다. 민감한 정보는 환경 변수에 보관하고 `Deno.env.get`으로 읽으세요:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

이는 Val Town에서 두 번째 이유 때문에 평소보다 더 중요합니다: **val을 리믹스하면 환경 변수 키는 복사되지만 값은 복사되지 않습니다.** 환경 변수에 보관된 비밀은 다른 사람의 계정으로 val이 이동할 때 따라가지 않습니다. 파일에 기록된 비밀은 따라갑니다.

`"demo"`로 되돌리면 자체 테넌트를 설정하기 전에 val을 리믹스하는 모든 사용자가 정상적으로 작동합니다.

## EU 계정

계정, 데이터 및 키는 하나의 지역에 존재합니다. 귀하의 계정이 `eu.fastcomments.com`에서 생성된 경우 모든 위젯 구성에도 `region: "eu"`가 필요하며, 스크립트는 `cdn-eu.fastcomments.com`에서 로드됩니다. 그렇지 않으면 두 설정을 그대로 두세요.

---