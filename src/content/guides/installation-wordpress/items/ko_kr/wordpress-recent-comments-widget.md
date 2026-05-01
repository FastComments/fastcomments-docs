최근 댓글 위젯은 사이트 전체에 게시된 가장 최근의 댓글을 표시합니다. 사이드바, 푸터 또는 새 활동을 노출해 추가 읽기를 유도하려는 곳에 유용합니다.

## 옵션

- **제목** (선택사항): 목록 위에 표시되는 제목입니다. 기본값은 "최근 댓글"입니다.
- **표시 수** (선택사항): 표시할 댓글 수입니다. 범위는 1에서 50입니다. 기본값은 5입니다.

## 추가 방법

### 게시물 또는 페이지 내

블록 편집기에서 **숏코드(Shortcode)** 블록을 추가하고 붙여넣으세요:

[inline-code-attrs-start title = '최근 댓글 숏코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

`count` 속성은 1에서 50 사이의 값을 허용합니다.

### 사이드바 또는 푸터(클래식 테마)

워드프레스 관리자에서 **외모 > 위젯**으로 이동하세요. 블록 삽입기에서 "FastComments"를 검색하고 **FastComments: Recent Comments**를 선택하세요. 사이드바, 헤더 또는 푸터 영역으로 드래그한 다음 위젯 패널에서 제목과 표시 수를 구성하세요.

### 블록 테마(전체 사이트 편집)

**외모 > 편집기**에서 **사이트 편집기**를 여세요. 위젯을 표시할 템플릿 부분으로 이동한 다음 **Legacy Widget** 블록을 삽입하고 드롭다운에서 **FastComments: Recent Comments**를 선택하세요.

## 문제 해결

위젯은 FastComments 설정이 완료되고 tenant ID가 저장된 후에만 렌더링됩니다. 위젯 영역이 비어 있으면 워드프레스 관리자에서 **FastComments** 설정을 완료하고 페이지를 새로고침하세요.