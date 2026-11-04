FastComments에서는 아티클 ID, 즉 댓글이 연결되는 페이지를 URL ID라고 부릅니다. URL이거나 ID일 수 있기 때문입니다.
URL ID는 다음과 같이 정의합니다. 컴포넌트는 config 객체의 변경 사항을 감시하고 다시 로드하므로 URL ID를 업데이트할 수 있습니다.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### 계정 리전 (주의: EU 고객)

계정이 EU에 위치한 경우 위젯 구성에서 `region = 'eu'`를 설정하세요. 예를 들면 다음과 같습니다:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

그렇지 않은 경우 `region`을 정의할 필요가 없습니다.