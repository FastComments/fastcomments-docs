`tenantId: "demo"`는 공유 공개 샌드박스입니다. 가입 없이 바로 사용할 수 있어 예제에서 이 값을 사용하지만, FastComments를 사용해 보는 모든 사람이 같은 스레드에 글을 쓰고 누구나 해당 내용을 관리할 수 있습니다. 중요한 것을 게시하기 전에 반드시 변경하세요.

테넌트 ID는 [API secret 페이지](https://fastcomments.com/auth/my-account/api-secret)에서 확인할 수 있습니다.

테넌트 ID는 공개 정보이며 브라우저 코드에 포함됩니다. API secret은 그렇지 않으며, 이 페이지의 어떤 내용도 API secret을 필요로 하지 않습니다.

## 환경 변수에서 읽기

Val Town의 val은 무료 티어에서 공개되므로 소스 코드가 전 세계에 노출됩니다. 민감한 정보는 환경 변수에 보관하고 `Deno.env.get`으로 읽으세요:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// eu.fastcomments.com에서 생성된 계정만 이 값을 "eu"로 설정합니다.
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Val Town에서는 두 번째 이유로 인해 이것이 평소보다 더 중요합니다: **val을 리믹스하면 환경 변수의 키는 복사되지만 값은 복사되지 않습니다.** 환경 변수에 보관된 secret은 다른 사람의 계정으로 val과 함께 전달되지 않습니다. 파일에 작성된 secret은 전달됩니다.

`"demo"`로 폴백하면 자신의 테넌트를 설정하기 전에 val을 리믹스하는 사람도 val을 계속 사용할 수 있습니다.

## EU 계정

계정, 데이터, 키는 하나의 리전에 존재합니다. 계정이 `eu.fastcomments.com`에서 생성되었다면 모든 위젯 설정에 `region: "eu"`가 필요하고, 스크립트는 `cdn-eu.fastcomments.com`에서 로드됩니다. 그렇지 않다면 둘 다 그대로 두세요.