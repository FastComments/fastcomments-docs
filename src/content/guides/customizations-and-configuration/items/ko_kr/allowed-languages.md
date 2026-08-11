---
기본적으로 FastComments는 댓글에 사용되는 언어를 제한하지 않습니다.  

커뮤니티에서 사용하는 언어를 제한하는 것이 바람직할 수 있습니다.

코드 없이 위젯 사용자 정의 페이지에서 구성할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='위젯 사용자 정의 페이지에서 댓글에 사용할 수 있는 언어를 제한하기 위한 허용된 언어 선택기'; title='허용된 언어' app-screenshot-end]

시스템은 댓글을 분석하여 언어를 판단한 다음 허용된 목록과 일치시킵니다.

댓글이 허용되지 않은 언어로 작성된 경우, 현지화된 오류 메시지가 표시됩니다.