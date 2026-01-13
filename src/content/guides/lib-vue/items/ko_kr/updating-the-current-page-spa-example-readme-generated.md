---
FastComments에서는 댓글이 연결되는 기사 ID 또는 페이지를 URL ID라고 합니다. 이는 URL이거나 ID일 수 있습니다.
URL ID는 다음과 같이 정의합니다. 컴포넌트는 config 객체의 변화를 감시하여 리로드하므로, "url"과 "urlId" 설정만 업데이트하면 됩니다.

전체 작동 예제는 [여기](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts)에서 확인하세요.

페이지네이션 예제를 다음 명령으로 실행하세요:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### 계정 지역 (주의: EU 고객)

계정이 EU에 있는 경우 위젯 구성에서 `region = 'eu'`로 설정하세요. 예를 들면:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

그렇지 않으면 `region`을 정의할 필요가 없습니다.
---