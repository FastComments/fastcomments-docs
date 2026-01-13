---
기본적으로 FastComments는 댓글 작성에 사용하는 언어를 제한하지 않습니다. 

커뮤니티에서 사용하는 언어를 제한하는 것이 바람직할 수 있습니다.

이 설정은 위젯 맞춤 설정 페이지에서 코드 없이 구성할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

시스템은 댓글을 분석하여 언어를 식별한 후 허용된 목록과 대조합니다.

댓글이 허용되지 않은 언어로 작성된 경우, 해당 언어로 된 오류 메시지가 표시됩니다. 

---