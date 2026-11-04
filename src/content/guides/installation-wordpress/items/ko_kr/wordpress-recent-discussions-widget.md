Recent Discussions 위젯은 사이트에서 가장 최근 댓글 활동이 있는 페이지를 표시합니다. 아직 활발히 대화가 이어지고 있는 스레드를 강조하는 데 유용하여, 방문자가 조용한 페이지에 머무는 대신 활성화된 대화로 다시 참여할 수 있습니다.

## 옵션

- **Title** (선택 사항): 목록 위에 표시되는 제목입니다. 기본값은 "Recent Discussions"입니다.
- **Count** (선택 사항): 표시할 토론 수입니다. 범위는 1부터 50까지이며, 기본값은 20입니다.

## 추가하는 방법

### 게시물 또는 페이지 내부

블록 에디터에서 **Shortcode** 블록을 추가하고 다음을 붙여넣으세요:

[inline-code-attrs-start title = 'Recent Discussions 숏코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

`count` 속성은 1부터 50 사이의 값을 허용합니다.

### 사이드바 또는 푸터에 추가 (클래식 테마)

WordPress 관리자 화면에서 **Appearance > Widgets**으로 이동하세요. 블록 삽입기에서 "FastComments"를 검색하고 **FastComments: Recent Discussions**를 선택하세요. 사이드바, 헤더 또는 푸터 영역으로 드래그한 후 위젯 패널에서 제목과 개수를 설정하세요.

### 블록 테마에서 추가 (전체 사이트 편집)

**Appearance > Editor** 아래의 **Site Editor**를 여세요. 위젯이 표시되어야 하는 템플릿 파트로 이동하여 **Legacy Widget** 블록을 삽입하고 드롭다운에서 **FastComments: Recent Discussions**를 선택하세요.

## 문제 해결

위젯은 FastComments 설정이 완료되고 테넌트 ID가 저장된 후에만 렌더링됩니다. 위젯 영역이 비어 있다면, WordPress 관리자 화면의 **FastComments**에서 설정을 완료하고 페이지를 새로고침하세요.

토론 정렬이 오래된 것처럼 보인다면, 해당 페이지들이 FastComments 대시보드에서 동기화를 마쳤는지 확인하세요. 위젯은 실시간 데이터를 읽어오므로, 새로 가져온 댓글이 표시되기까지 잠시 시간이 걸릴 수 있습니다.