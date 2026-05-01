The Recent Discussions widget displays the pages on your site with the most recent comment activity. It's useful for highlighting threads that are still being added to, so visitors can jump back into active conversations rather than landing on quiet pages.

## 옵션

- **Title** (선택 사항): 목록 위에 표시되는 제목입니다. 기본값은 "Recent Discussions".
- **Count** (선택 사항): 표시할 토론의 수입니다. 범위는 1에서 50입니다. 기본값은 20입니다.

## 추가 방법

### 게시물 또는 페이지 안에서

블록 편집기에서 **Shortcode** 블록을 추가하고 다음을 붙여넣으세요:

[inline-code-attrs-start title = '최근 토론 숏코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### 사이드바 또는 푸터(클래식 테마)

워드프레스 관리자에서 **Appearance > Widgets**로 이동하세요. 블록 삽입기에서 "FastComments"를 검색하고 **FastComments: Recent Discussions**를 선택하세요. 사이드바, 헤더 또는 푸터 영역으로 드래그한 다음 위젯 패널에서 Title 및 Count를 구성하세요.

### 블록 테마(전체 사이트 편집)에서

**Appearance > Editor** 아래의 **Site Editor**를 엽니다. 위젯을 표시할 템플릿 부분으로 이동하여 **Legacy Widget** 블록을 삽입하고 드롭다운에서 **FastComments: Recent Discussions**를 선택하세요.

## 문제 해결

위젯은 FastComments 설정이 완료되고 tenant ID가 저장된 후에만 렌더링됩니다. 위젯 영역이 비어 있다면 워드프레스 관리자에서 **FastComments** 설정을 완료하고 페이지를 새로고침하세요.

토론 정렬 순서가 오래된 것처럼 보이면 기본 페이지들이 FastComments 대시보드에서 동기화가 완료되었는지 확인하세요. 위젯은 실시간 데이터를 읽기 때문에 새로 가져온 댓글이 나타나려면 잠시 시간이 걸릴 수 있습니다.