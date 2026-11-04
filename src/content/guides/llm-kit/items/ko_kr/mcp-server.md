FastComments는 호스팅된 Model Context Protocol(MCP) 서버를 운영하여 AI 어시스턴트와 에이전트 클라이언트가 FastComments API를 직접 호출할 수 있도록 합니다. MCP 서버가 노출하는 모든 도구는 공개 OpenAPI 스펙에서 자동 생성되므로, REST API가 할 수 있는 모든 작업을 MCP 클라이언트도 수행할 수 있습니다.

엔드포인트는 상태 비저장(stateless)이며 streamable-HTTP 기반입니다. 유지해야 할 세션이 없고 클라이언트별 서버 측 상태도 없습니다.

### 엔드포인트

[inline-code-attrs-start title = 'MCP 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### OAuth로 연결

OAuth를 지원하는 원격 서버를 지원하는 모든 MCP 클라이언트(Claude, ChatGPT, Claude Code, Cursor 등)는 FastComments 측에서 별도 설정 없이 위 엔드포인트에 연결할 수 있습니다. 클라이언트는 Dynamic Client Registration을 통해 자신을 등록하거나 Client ID Metadata Document로 자신을 식별하고, 브라우저를 열어 FastComments에 로그인하고 접근을 승인하도록 하며, 로그인한 계정에 바인딩된 토큰을 받습니다.

디스커버리 문서는 표준 위치에 있습니다:

[inline-code-attrs-start title = '디스커버리'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

연결을 승인하려면 사용자 계정에 대한 API Admin 권한이 필요합니다. 여러 계정을 관리하는 경우 승인하기 전에 대시보드에서 올바른 계정으로 전환하세요.

클라이언트는 `read` 스코프, `write` 스코프 또는 둘 다를 요청할 수 있습니다. 아무것도 요청하지 않는 클라이언트는 둘 다 받습니다. 데이터를 변경하는 도구는 읽기 전용 토큰에는 제공되지 않습니다.

대시보드에는 바로 붙여넣을 수 있는 스니펫이 포함된 설정 도우미가 있습니다. **Integrate -> MCP Server**를 열거나 다음 URL로 직접 방문하세요:

[inline-code-attrs-start title = '설정 페이지'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

한 번의 명령으로 FastComments 서버를 등록한 다음, 세션 내에서 `/mcp`를 실행하여 로그인하고 사용 가능한 도구를 나열하세요:

[inline-code-attrs-start title = 'Claude Code 설정'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor 및 기타 설정 파일 기반 클라이언트

클라이언트의 MCP 서버 설정에 이 블록을 추가하세요(Cursor의 경우 `mcp.json`). 클라이언트는 처음 사용 시 브라우저를 열어 로그인합니다.

[inline-code-attrs-start title = 'MCP 클라이언트 설정'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### 접근 권한 해지

승인된 모든 연결은 대시보드의 **Integrate -> Connected Apps**에 나열됩니다. 연결을 해지하면 해당 애플리케이션이 보유한 모든 토큰이 무효화됩니다. 애플리케이션은 연결 시 자신을 등록하며 FastComments는 이를 검토하지 않으므로, 인식하지 못하는 애플리케이션은 해지하세요.

### REST API에서 토큰 사용

MCP 클라이언트가 얻은 액세스 토큰은 일반적인 FastComments API 자격 증명입니다. 모든 `/api/v1` 엔드포인트에서 bearer 토큰으로 작동하므로, MCP를 통해 연결된 애플리케이션은 REST API를 직접 호출할 수도 있습니다:

[inline-code-attrs-start title = 'Bearer 토큰'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

테넌트는 토큰에 의해 암시됩니다. `tenantId`를 전달할 수는 있지만 일치해야 합니다. `GET` 요청에는 `read` 스코프가 필요하고 그 외 모든 요청에는 `write` 스코프가 필요합니다.

### API 키로 연결

헤드리스 서버와 같이 브라우저 로그인을 완료할 수 없는 클라이언트는 대신 API 키로 인증할 수 있습니다. `tenantId`와 `API_KEY`를 쿼리 파라미터로 전달하거나, 클라이언트가 사용자 지정 헤더를 지원하는 경우 `x-tenant-id` 및 `x-api-key` HTTP 헤더로 전달하세요:

[inline-code-attrs-start title = 'API 키 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

설정 페이지에서 각 API 키에 대해 이 URL을 생성해 줍니다.

### 보안

API 키가 포함된 엔드포인트 URL은 비밀 정보입니다. 공개 채팅, 스크린샷 또는 커밋에 붙여넣지 마세요. 키가 노출된 경우 대시보드의 API Keys 페이지에서 키를 교체(rotate)하세요. OAuth 토큰은 하나의 애플리케이션에 바인딩되고 Connected Apps에서 해지할 수 있으므로 이러한 위험이 없습니다.