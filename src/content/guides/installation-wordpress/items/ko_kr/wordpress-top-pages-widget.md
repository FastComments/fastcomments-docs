Top Pages 위젯은 사이트에서 댓글이 가장 많은 페이지를 표시합니다. 새로운 방문자에게 가장 반응이 좋은 콘텐츠를 보여주고 사이트 체류 시간을 늘리는 데 유용합니다.

## Options

- **Title** (optional): 목록 위에 표시되는 제목입니다. 기본값은 "Top Pages"입니다.

Top Pages 위젯은 사용 가능한 데이터에 따라 자체 레이아웃을 선택하며 count 속성을 허용하지 않습니다.

## How to Add It

### Inside a Post or Page

블록 편집기에서 **Shortcode** 블록을 추가하고 다음을 붙여넣으세요:

[inline-code-attrs-start title = 'Top Pages 숏코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### In a Sidebar or Footer (Classic Themes)

WordPress 관리자에서 **Appearance > Widgets**로 이동하세요. 블록 삽입기에서 "FastComments"를 검색하고 **FastComments: Top Pages**를 선택하세요. 사이드바, 헤더 또는 푸터 영역으로 끌어다 놓은 다음 위젯 패널에서 제목을 설정하세요.

### In a Block Theme (Full Site Editing)

**Appearance > Editor**에서 **Site Editor**를 엽니다. 위젯을 넣을 템플릿 부분으로 이동하여 **Legacy Widget** 블록을 삽입한 다음 드롭다운에서 **FastComments: Top Pages**를 선택하세요.

## Troubleshooting

이 위젯은 FastComments 설정이 완료되어 tenant ID가 저장된 후에만 렌더링됩니다. 위젯 영역이 비어 있으면 WordPress 관리자에서 **FastComments**에서 설정을 완료한 뒤 페이지를 새로고침하세요.