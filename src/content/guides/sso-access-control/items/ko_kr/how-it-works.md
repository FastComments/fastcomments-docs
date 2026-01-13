FastComments 액세스 제어는 원하는 그룹에 `Pages` 및 `Users`를 할당하는 방식으로 작동합니다.

그룹은 단순히 문자열 식별자이며, 예: `GREEN` 또는 `abc-123`.

`Users`와 `Pages`는 하나의 그룹에만 제한되지 않습니다. 각각 `100` 및 `1000`개의 그룹으로 제한됩니다. 

#### 권한이 없는 페이지에 접근

사용자가 접근 권한이 없는 페이지에 접근하려고 하면, 아래와 같은 오류 메시지가 표시됩니다:

<div class="screenshot white-bg">
    <div class="title">권한 실패 예시</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Authorization Failure Example" />
</div>

메시지 텍스트는 사용자 지정할 수 있습니다.

---