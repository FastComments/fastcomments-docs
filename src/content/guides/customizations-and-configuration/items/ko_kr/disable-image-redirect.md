---
[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자가 이미지를 업로드할 수 있도록 허용합니다. 사용자가 그 이미지를 클릭하면, FastComments는 기본적으로,
새 탭을 열어 해당 이미지를 전체 보기로 표시합니다. 이 플래그를 true로 설정하면 이 동작이 비활성화됩니다:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

이미지 클릭을 직접 캡처할 계획이 없다면 (참조: [onImageClicked](#callbacks)), 이 동작을 일부 스타일과 결합하는 것을 권장합니다
이미지가 클릭 가능한 것처럼 보이는 외관을 제거하기 위해.

---