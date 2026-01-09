[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments는 사용자 맞춤형으로 설계되었으며, 위젯에서 사용하는 글꼴도 예외는 아닙니다.

기본적으로 FastComments는 가능한 한 다양한 기기에서 보기 좋게 표시되도록 `system font stack`을 사용합니다.

자신의 글꼴을 정의하려면 [맞춤 CSS 문서](/guide-customizations-and-configuration.html#custom-css)를 참조하세요.

해당 문서에서는 원하는 글꼴을 지정할 수 있는 맞춤 CSS 정의 방법을 확인할 수 있습니다.

#### 글꼴을 정의하는 방법

글꼴을 재정의하려면 `.fast-comments, textarea` 선택자를 사용하여 CSS를 정의하는 것을 권장합니다. 예:

[inline-code-attrs-start title = '커스텀 외부 폰트 예제'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---