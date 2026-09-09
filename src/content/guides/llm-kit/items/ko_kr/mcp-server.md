FastComments는 호스팅된 Model Context Protocol (MCP) 서버를 실행하여 AI 어시스턴트와 에이전트 클라이언트가 FastComments API를 직접 호출할 수 있도록 합니다. MCP 서버가 제공하는 모든 도구는 공개 OpenAPI 사양에서 자동 생성되므로 REST API가 할 수 있는 모든 작업을 MCP 클라이언트도 수행할 수 있습니다.

엔드포인트는 무상태이며 스트리밍 가능한 HTTP 기반입니다. 유지해야 할 세션이 없으며 클라이언트당 서버 측 상태도 없습니다.

### Endpoint

[inline-code-attrs-start title = 'MCP 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

OAuth(Claude, ChatGPT, Claude Code, Cursor 등)를 지원하는 모든 MCP 클라이언트는 FastComments 측에서 별도의 설정 없이 위 엔드포인트에 연결할 수 있습니다. 클라이언트는 동적 클라이언트 등록(Dynamic Client Registration)이나 클라이언트 ID 메타데이터 문서를 통해 자신을 등록하고, 브라우저를 열어 FastComments에 로그인하고 접근을 승인한 뒤, 로그인한 계정에 연결된 토큰을 받습니다.

디스커버리 문서는 표준 위치에 있습니다:

[inline-code-attrs-start title = '디스커버리'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

연결을 승인하려면 해당 계정에 대한 API 관리자 권한이 필요합니다. 여러 계정을 관리하는 경우, 승인하기 전에 대시보드에서 올바른 계정으로 전환하십시오.

클라이언트는 `read` 범위, `write` 범위 또는 둘 다를 요청할 수 있습니다. 아무 것도 요청하지 않는 클라이언트는 두 범위를 모두 받습니다. 데이터를 변경하는 도구는 읽기 전용 토큰에는 제공되지 않습니다.

대시보드에는 바로 복사해 붙일 수 있는 스니펫이 포함된 설정 도우미가 있습니다. **Integrate -> MCP Server**를 열거나 직접 방문하십시오:

[inline-code-attrs-start title = '설정 페이지'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

한 명령으로 FastComments 서버를 등록한 뒤, 세션 내에서 `/mcp`를 실행하여 로그인하고 사용 가능한 도구를 나열합니다:

[inline-code-attrs-start title = 'Claude Code 설정'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

이 블록을 클라이언트의 MCP 서버 구성(`Cursor의 경우 mcp.json`)에 추가하십시오. 클라이언트는 처음 사용할 때 브라우저를 열어 로그인합니다.

[inline-code-attrs-start title = 'MCP 클라이언트 구성'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Revoking access

승인된 모든 연결은 대시보드의 **Integrate -> Connected Apps**에 나열됩니다. 하나를 해제하면 해당 애플리케이션이 보유한 모든 토큰이 무효화됩니다. 애플리케이션은 연결 시 스스로 등록하며 FastComments는 이를 검토하지 않으므로, 인식하지 못하는 항목은 모두 해제하십시오.

### Using the token with the REST API

MCP 클라이언트가 얻은 액세스 토큰은 일반 FastComments API 자격 증명과 동일합니다. 이는 모든 `/api/v1` 엔드포인트에서 베어러 토큰으로 작동하므로, MCP를 통해 연결된 애플리케이션도 REST API를 직접 호출할 수 있습니다:

[inline-code-attrs-start title = '베어러 토큰'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

테넌트는 토큰에 의해 암시됩니다. `tenantId`를 전달할 수도 있지만 토큰과 일치해야 합니다. `GET` 요청은 `read` 범위가 필요하고, 그 외의 모든 요청은 `write` 범위가 필요합니다.

### Connect with an API key

브라우저 로그인 절차를 수행할 수 없는 클라이언트(예: 헤드리스 서버)는 대신 API 키로 인증할 수 있습니다. `tenantId`와 `API_KEY`를 쿼리 매개변수로 전달하거나, 클라이언트가 사용자 정의 헤더를 지원하는 경우 `x-tenant-id`와 `x-api-key` HTTP 헤더로 전달하십시오:

[inline-code-attrs-start title = 'API 키 엔드포인트'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

설정 페이지는 각 API 키에 대해 이 URL을 생성합니다.

### Security

API 키가 포함된 엔드포인트 URL은 비밀 정보이므로 공개 채팅, 스크린샷 또는 커밋에 붙여넣지 마십시오. 키가 노출된 경우 대시보드의 API Keys 페이지에서 키를 교체하십시오. OAuth 토큰은 하나의 애플리케이션에만 연결되어 있으며 Connected Apps에서 해제할 수 있기 때문에 이러한 위험이 없습니다.