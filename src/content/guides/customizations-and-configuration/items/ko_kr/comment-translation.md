---
기본적으로 FastComments는 위젯 자체 인터페이스(버튼, 레이블, 오류 메시지)를 뷰어의 언어로 번역하지만, 댓글은 작성된 그대로 표시됩니다.

Comment Translation은 댓글 자체의 번역을 추가하여, 여러 언어로 작성하는 커뮤니티가 모든 댓글을 자신의 언어로 읽을 수 있게 합니다.

두 가지 모드가 있습니다:

- **Automatic**: 위젯 언어와 다른 언어로 작성된 댓글은 번역되어 표시됩니다. 댓글 아래에 "Translated from Spanish"와 같은 문구가 나타나며, **Show original** 링크를 통해 번역된 텍스트와 원본 텍스트를 전환할 수 있습니다.
- **Manual**: 모든 댓글에 **Translate** 링크가 표시됩니다. 이를 클릭하면 해당 댓글이 뷰어의 언어로 번역되며, 이후 동일한 **Show original** 토글이 제공됩니다. 이미 뷰어 언어로 된 댓글은 대신 해당 언어임을 표시합니다.

대상 언어는 위젯의 언어입니다: 동일한 커스터마이징 규칙에 설정된 [로케일 / 언어](#locale) 설정이 있으면 그 값을 사용하고, 없으면 방문자의 브라우저 언어를 사용합니다. 방문자에게 별도의 계정, 확장 프로그램 또는 추가 소프트웨어가 필요하지 않으며, 임베드된 위젯과 독립형 댓글 페이지 모두에서 동일하게 작동합니다.

이 기능은 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">커스터마이징 규칙</a>당 활성화되므로 전체 사이트, 하나의 도메인 또는 단일 페이지에 적용할 수 있습니다. 기본적으로 비활성화되어 있습니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='위젯 커스터마이징 페이지에서 Off, Automatic 및 Manual 옵션이 있는 댓글 번역 드롭다운'; title='댓글 번역' app-screenshot-end]

번역은 서버 측 설정입니다. 위젯의 JavaScript 구성에 `commentTranslation`을 전달해도 활성화되지 않으며, 해당 규칙이 이를 활성화합니다.

각 댓글은 대상 언어당 최대 한 번만 번역되며, 결과는 댓글과 함께 저장됩니다. 따라서 천 명의 스페인어 사용자가 읽는 댓글도 한 번만 번역됩니다. 작성자나 모더레이터가 댓글을 편집하면 저장된 번역이 폐기되고 다음 조회 시 다시 번역됩니다. 번역된 텍스트는 원본 댓글과 동일한 정화 및 링크 규칙을 적용받습니다. 너무 길거나 언어를 감지할 수 없는 댓글은 번역되지 않은 상태로 표시됩니다.

사용량은 [AI 에이전트](/guide-ai-agents.html#plans-and-eligibility)와 동일한 요율로 LLM 토큰으로 청구되며, 청구 상세 내 동일한 위치에 표시됩니다.
---