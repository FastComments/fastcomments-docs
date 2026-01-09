[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments 댓글 위젯은 대부분의 사이트에서 다크 모드를 자동으로 감지합니다.

다크 모드가 감지되면 FastComments는 흰 배경의 검은색 텍스트에서 검은 배경의 흰색 텍스트로 전환합니다. 이미지도 변경됩니다.

페이지 로드 시, 위젯은 댓글 위젯 뒤에 있는 페이지 배경이 얼마나 어두운지를 판단하려고 시도합니다. 이는
페이지가 흰 배경일 수 있지만, 댓글 위젯을 검은색 배경의 컨테이너 안에 넣으면 댓글을 읽기 쉽게 하기 위해 다크 모드가
여전히 자동으로 활성화되어야 함을 의미합니다.

그러나 "휘도(luminance)"를 결정하는 방식에 의존하는 감지 메커니즘은 원할 때 다크 모드를 활성화하지 않을 수 있습니다. 강제로 활성화하려면
*hasDarkBackground* 플래그를 다음과 같이 true로 설정하십시오:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---