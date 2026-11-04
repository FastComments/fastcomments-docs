Top Pages 위젯은 사이트에서 댓글이 가장 많은 페이지를 표시합니다. 가장 참여도가 높은 콘텐츠를 새 방문자에게 노출하고 사이트 체류 시간을 늘리는 데 유용합니다.

## 옵션

- **Title**(선택 사항): 목록 위에 표시되는 제목입니다. 기본값은 "Top Pages"입니다.

Top Pages 위젯은 사용 가능한 데이터를 기반으로 자체 레이아웃을 선택하며 count 속성을 받지 않습니다.

## 추가하는 방법

### 게시물 또는 페이지 내부

블록 에디터에서 **Shortcode** 블록을 추가하고 다음을 붙여넣으세요:

[inline-code-attrs-start title = 'Top Pages 숏코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### 사이드바 또는 푸터에 (클래식 테마)

WordPress 관리자 화면에서 **Appearance > Widgets**로 이동하세요. 블록 삽입기에서 "FastComments"를 검색하고 **FastComments: Top Pages**를 선택하세요. 사이드바, 헤더 또는 푸터 영역으로 드래그한 후 위젯 패널에서 제목을 설정하세요.

### 블록 테마에서 (전체 사이트 편집)

**Appearance > Editor** 아래의 **Site Editor**를 여세요. 위젯이 표시되어야 하는 템플릿 파트로 이동하여 **Legacy Widget** 블록을 삽입하고 드롭다운에서 **FastComments: Top Pages**를 선택하세요.

## 문제 해결

이 위젯은 FastComments 설정이 완료되고 테넌트 ID가 저장된 후에만 렌더링됩니다. 위젯 영역이 비어 있다면 WordPress 관리자 화면의 **FastComments**에서 설정을 완료한 후 페이지를 새로고침하세요.