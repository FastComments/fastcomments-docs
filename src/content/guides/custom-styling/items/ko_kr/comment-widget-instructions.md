## 댓글 위젯 스타일 사용자 지정 방법

댓글 위젯 스타일은 다음 두 가지 방법으로 사용자 지정할 수 있습니다:

### 옵션 1: customCSS 매개변수 사용

위젯을 초기화할 때 `customCSS` 매개변수에 사용자 정의 CSS를 문자열로 전달하세요:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### 옵션 2: 관리자 대시보드 사용

1. 관리자 대시보드에서 [위젯 사용자 지정 페이지](https://fastcomments.com/auth/my-account/customize-widget)로 이동하세요
2. "Advanced" 아래의 "Custom CSS" 섹션으로 스크롤하세요
3. 사용자 정의 CSS를 입력하세요
4. "Save"를 클릭하세요

사용자 정의 CSS는 사이트의 모든 댓글 위젯에 적용됩니다.

## 팁

- 필요할 경우 기본 스타일을 재정의하려면 `!important`를 사용하세요
- 사이트의 다른 부분에 영향을 주지 않도록 특정 선택자를 타겟팅하세요
- 호환성을 위해 다양한 브라우저에서 CSS를 테스트하세요
- 위젯은 표준 CSS를 사용합니다 - 별도의 전처리기가 필요하지 않습니다