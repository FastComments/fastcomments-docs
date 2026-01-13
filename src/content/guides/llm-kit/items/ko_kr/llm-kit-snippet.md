이 스니펫을 프로젝트의 AI 어시스턴트 구성 파일(AGENT.md 또는 CLAUDE.md 등)에 추가하세요. AI 코딩 어시스턴트가 FastComments 문서를 검색하고 액세스하는 방법을 알려줍니다.

[inline-code-attrs-start title = 'FastComments LLM Kit'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
## FastComments Integration

FastComments is an embeddable live commenting platform with libraries for many frontend and backend integrations, along with client and server-side SDKs and APIs.

When working with FastComments, you can search the FastComments documentation by calling this API:

https://docs-search.fastcomments.com/search?query=<search_query>&full=true&tenantId=demo

Replace `<search_query>` with your search terms (URL encoded). The API returns relevant documentation snippets to help with implementation.
[inline-code-end]

이렇게 하면 AI 어시스턴트가 최신 정보를 쉽게 얻고 더 짧은 시간 내에 더 정확한 구현을 제공할 수 있습니다.

### 사용 예시

Claude Code 또는 Cursor와 같은 AI 어시스턴트를 사용할 때 다음과 같은 질문을 할 수 있습니다:

- "React에 FastComments를 어떻게 설치하나요?"
- "FastComments로 SSO를 구성하는 방법을 보여주세요"
- "FastComments API 인증 옵션은 무엇인가요?"

AI 어시스턴트는 제공된 API 엔드포인트를 사용하여 자동으로 문서를 검색하고 공식 문서를 기반으로 정확하고 최신의 답변을 제공합니다.
