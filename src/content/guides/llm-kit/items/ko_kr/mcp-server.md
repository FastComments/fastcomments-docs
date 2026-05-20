FastComments는 호스팅된 Model Context Protocol (MCP) 서버를 운영하여 AI 코딩 어시스턴트 및 에이전트형 클라이언트가 FastComments API를 직접 호출할 수 있도록 합니다. MCP 서버가 노출하는 모든 도구는 공개 OpenAPI 스펙에서 자동 생성되므로 REST API가 할 수 있는 모든 작업을 MCP 클라이언트도 수행할 수 있습니다.

해당 엔드포인트는 무상태(stateless)이며 스트리밍 가능한 HTTP 기반입니다. 유지해야 할 세션이 없고, 클라이언트 등록 단계가 없으며, 클라이언트별 서버 측 상태가 없습니다.

### 엔드포인트

[inline-code-attrs-start title = 'MCP 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

인증은 REST API와 동일한 API 키를 사용합니다. 클라이언트가 커스텀 헤더를 지원하면 `tenantId`와 키를 `x-tenant-id` 및 `x-api-key` HTTP 헤더로 전달할 수도 있습니다.

### 미리 채워진 설정

대시보드에는 인기 있는 MCP 클라이언트용으로 URL과 바로 붙여넣을 수 있는 구성 스니펫을 생성하는 설정 도우미가 있습니다. 계정 대시보드에서 **Integrate -> MCP Server**를 열거나 직접 방문하세요:

[inline-code-attrs-start title = '설정 페이지'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

드롭다운에서 사용할 API 키를 선택한 다음, 생성된 스니펫 중 하나를 복사하세요.

### Claude Code

한 명령으로 FastComments 서버를 등록합니다:

[inline-code-attrs-start title = 'Claude Code 설정'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

등록한 후에는 Claude Code 세션 내에서 `/mcp`를 실행하여 연결을 확인하고 사용 가능한 도구를 나열하세요.

### Claude Desktop / Cursor

이 블록을 클라이언트의 MCP 서버 구성에 추가하세요(Claude Desktop의 경우 `claude_desktop_config.json`, Cursor의 경우 `mcp.json`):

[inline-code-attrs-start title = 'MCP 클라이언트 구성'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### 보안

API 키가 URL에 포함되어 있습니다. URL을 비밀처럼 취급하세요: 공개 채팅, 스크린샷, 커밋 등에 붙여넣지 마십시오. 키가 노출된 경우, 대시보드의 API 키 페이지에서 키를 교체하세요.