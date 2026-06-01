---
테넌트 ID를 한 번만 `_config.yml`에 설정하세요:

```yaml
fastcomments:
  tenant_id: demo
```

그런 다음 위젯을 표시할 레이아웃, 게시물 또는 페이지 등 원하는 곳에 태그를 추가하세요:

```liquid
{% raw %}{% fastcomments %}{% endraw %}
```

이게 전부입니다. `demo`를 FastComments 테넌트 ID로 바꾸세요 (다음에서 찾을 수 있습니다
[설정 > API/SSO](https://fastcomments.com/auth/my-account/api)).
---