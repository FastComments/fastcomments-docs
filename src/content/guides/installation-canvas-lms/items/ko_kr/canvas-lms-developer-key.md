#### Open Developer Keys in Canvas

관리자 계정으로 Canvas에 로그인합니다. 왼쪽 사이드바에서 **Admin** > 계정을 선택 > **Developer Keys**로 이동합니다.

#### Create an LTI Developer Key

**+ Developer Key**를 클릭하고 **LTI Key**를 선택합니다.

In the configuration form:

1. 왼쪽의 **Redirect URIs** 필드에 FastComments 설정 페이지의 **Launch URL**을 붙여넣습니다.
2. 오른쪽에서 **Method**를 **Enter URL**로 설정합니다.
3. FastComments에서 복사한 **Configuration URL**을 **JSON URL** 필드에 붙여넣습니다.
4. Canvas가 LTI 구성을 자동으로 불러옵니다.
5. 키 이름을 지정합니다(예: "FastComments").
6. **Save**를 클릭합니다.

#### Enable the Developer Key

저장한 후 새 키가 Developer Keys 표에 **State**가 **OFF**로 표시됩니다. 토글을 클릭하여 **ON**으로 설정합니다. Canvas가 확인을 요청할 수 있으며 — **Allow**를 클릭하여 키를 활성화합니다.

#### Copy the Client ID

Developer Keys 표의 Details 열에 숫자 형식의 **Client ID**가 표시됩니다(예: `17000000000042`). 이 번호를 복사하세요 — 다음 단계에서 필요합니다.