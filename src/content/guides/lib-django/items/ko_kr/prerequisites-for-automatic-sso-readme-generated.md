---
로그인한 사용자를 위젯에 자동으로 전달하려면, 태그가 요청(request)에서 현재 사용자를 읽어옵니다. 프로젝트에 아래 두 항목이 모두 포함되어 있는지 확인하세요(표준 Django 프로젝트에서는 기본적으로 활성화되어 있습니다):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

템플릿 컨텍스트에 요청이 없으면, 위젯은 익명 방문자용으로 렌더링됩니다. 항상 명시적으로 사용자를 전달할 수 있습니다: `{% fastcomments user=some_user %}`.
---