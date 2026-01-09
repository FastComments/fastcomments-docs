이제 맞춤 FastComments 코드를 생성합니다. 아래 마법사를 사용하여 GoHighLevel 사이트에서 FastComments가 작동하는 방식을 구성하세요:

[snippet id="gohighlevel-wizard"]

### 다른 댓글 박스 유형

사용되는 제품을 전환하려면 `TYPE = 'commenting'` 줄을 구성할 수 있습니다 (예: 스트리밍 채팅은 `live`로, 협업 채팅은 `collab`로 변경할 수 있습니다).

### 댓글 박스를 원하는 위치에 배치하기

페이지의 특정 부분에만 댓글 박스를 넣고 기본 위치에는 넣지 않으려는 경우를 가정해봅시다.
이 줄을 변경하세요:

    const TARGET_ELEMENT_ID = ''; // 타겟 div 모드 사용을 위해 설정

다음으로:

    const TARGET_ELEMENT_ID = 'fc_box'; // 타겟 div 모드 사용을 위해 설정

그런 다음 GHL 에디터에서 "code" 버튼을 클릭하고 댓글을 표시할 위치에 다음을 추가하세요:

[inline-code-attrs-start title = 'GoHighLevel FastComments DIV(영역)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### 페이지별로 다른 댓글 박스 유형

사용자가 텍스트의 일부를 강조 표시하고 토론하게 하거나 대신 스트리밍 채팅 UI를 사용하게 하려는 경우를 가정해봅시다.

먼저 위의 "댓글 박스를 원하는 위치에 배치하기" 섹션의 단계를 따르세요.

작은 스니펫에 `type="commenting"`이 있다는 점을 참고하세요.

예를 들어 collab 채팅을 활성화하려면 `type="collab"`로 변경하세요.

### 특정 페이지에만 표시

`TARGET_ELEMENT_ID`를 설정하지 않으면 대신 `VALID_PATTERNS` 변수를 구성하여 댓글을 표시할 URL 경로를 설정할 수 있습니다. 기본적으로 URL에 `/post`가 포함된 페이지에 표시됩니다.

### Collab 채팅 구성하기

Collab 채팅이 특정 영역 내의 HTML에 대해서만 협업 기능을 추가하도록 지정할 수 있습니다. 예를 들어 위의 푸터 코드를 추가한 다음 게시물/페이지 내용에 이 div를 추가하여 collab 채팅을 활성화한다고 가정해보겠습니다:

[inline-code-attrs-start title = '지정된 콘텐츠로 Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

그런 다음 `<div>` 내부의 단락 요소에는 collab 채팅이 활성화되며 페이지의 다른 부분에는 영향을 주지 않습니다. `<div>`에 아무 콘텐츠도 넣지 않으면 게시물 본문 전체에 collab 채팅이 활성화됩니다.