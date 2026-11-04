#### Canvas에서 개발자 키 열기

Canvas에 관리자로 로그인합니다. **Admin**(왼쪽 사이드바) > 계정 선택 > **Developer Keys**로 이동합니다.

#### LTI 개발자 키 생성

**+ Developer Key**를 클릭하고 **LTI Key**를 선택합니다.

구성 양식에서:

1. **Redirect URIs** 필드(왼쪽)에 FastComments 설정 페이지의 **Launch URL**을 붙여넣습니다.
2. 오른쪽에서 **Method**를 **Enter URL**로 설정합니다.
3. FastComments에서 복사한 **Configuration URL**을 **JSON URL** 필드에 붙여넣습니다.
4. Canvas가 LTI 구성을 자동으로 불러옵니다.
5. 키에 이름을 지정합니다(예: "FastComments").
6. **Save**를 클릭합니다.

#### 개발자 키 활성화

저장하면 새 키가 개발자 키 테이블에 나타나며 **State**가 **OFF**로 설정되어 있습니다. 토글을 클릭하여 **ON**으로 설정합니다. Canvas에서 확인을 요구할 수 있습니다 — **Allow**를 클릭하여 키를 활성화합니다.

#### 클라이언트 ID 복사

개발자 키 테이블의 Details 열에 숫자 형식의 **Client ID**가 표시됩니다(예: `17000000000042`). 이 번호를 복사합니다 - 다음 단계에서 필요합니다.