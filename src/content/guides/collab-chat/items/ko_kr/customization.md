---
### 다크 모드 지원

### 동적 다크 모드

사이트의 다크 모드가 body 요소에 `.dark` 클래스를 추가하여 제어되는 경우, Collab Chat UI는 재초기화 없이 자동으로 이를 반영합니다. 위젯의 스타일은 이 클래스의 존재에 반응하도록 설계되어 있습니다.

[inline-code-attrs-start title = '다크 모드 CSS 예제'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* 다크 모드 CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### CSS로 사용자 지정 스타일

하이라이트, 채팅 창 및 기타 요소의 외형을 CSS로 사용자 지정할 수 있습니다. 위젯은 스타일시트에서 타겟팅할 수 있는 특정 클래스를 추가합니다.

텍스트 하이라이트는 FastComments의 댓글 버블 스타일링 시스템을 사용하므로, 표준 댓글 위젯에 적용한 모든 사용자 정의는 Collab Chat에도 영향을 미칩니다.

### 상단 바 사용자 지정

상단 바에는 온라인 사용자 수와 토론 수가 표시됩니다. `topBarTarget`으로 커스텀 요소를 제공하여 위치를 사용자 지정할 수 있습니다:

[inline-code-attrs-start title = '사용자 정의 상단 바 위치'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

또는 `null`로 설정하여 완전히 비활성화할 수 있습니다:

[inline-code-attrs-start title = '상단 바 비활성화'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### 모바일 동작

화면 너비가 768px 미만인 경우 Collab Chat은 자동으로 모바일 최적화 레이아웃으로 전환합니다. 채팅 창은 텍스트 옆에 떠 있지 않고 전체 화면으로 표시되며, 더 즉각적인 상호작용을 위해 선택 지연이 제거됩니다.

이 동작은 내장되어 있으며 별도의 구성은 필요하지 않습니다. 위젯은 화면 크기를 자동으로 감지하고 그에 따라 조정합니다.

### 채팅 창 외형

채팅 창은 데스크톱에서 너비가 410px이고, 하이라이트된 텍스트를 가리키는 16px 화살표가 있습니다. 창은 뷰포트의 사용 가능한 공간에 따라 자동으로 위치를 지정하며, `to-right`, `to-left`, `to-top`, `to-bottom` 같은 위치 지정 클래스를 사용합니다.

이 창들의 색상, 글꼴, 간격 또는 기타 시각적 속성을 조정하기 위해 사용자 지정 CSS를 추가할 수 있습니다. 채팅 창은 표준 FastComments 위젯과 동일한 컴포넌트 구조를 사용하므로 전역 사용자 지정도 상속됩니다.

### 지역화

Collab Chat은 표준 FastComments 위젯과 동일한 모든 지역화 옵션을 지원합니다. UI 텍스트를 다른 언어로 표시하려면 `locale` 옵션을 설정하세요:

[inline-code-attrs-start title = '로케일 설정'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 스페인어
});
[inline-code-end]

FastComments는 수십 개의 언어를 지원합니다. 로케일 설정은 프롬프트, 버튼 및 플레이스홀더 텍스트를 포함한 모든 UI 텍스트에 영향을 줍니다.

### 상속된 사용자 지정 옵션

Collab Chat은 표준 댓글 위젯을 확장하므로, 기본 위젯의 모든 사용자 지정 옵션을 상속합니다. 여기에는 사용자 지정 CSS 클래스, 사용자 지정 번역, 아바타 사용자 지정, 날짜 형식 지정 등 많은 항목이 포함됩니다.

사용 가능한 전체 사용자 지정 옵션 목록은 FastComments의 메인 사용자 지정 문서를 참조하세요.

### 사용자 지정 폰트 사용

사이트에서 사용자 지정 폰트를 사용하는 경우 Collab Chat UI는 페이지의 CSS에서 해당 폰트를 상속합니다. 플로팅 채팅 창이 동일한 폰트를 사용하도록 하려면 위젯 커스터마이제이션 규칙을 생성하고 해당 규칙의 커스텀 CSS에서 `@import`로 폰트를 포함해야 할 수 있습니다.

---