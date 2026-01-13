FastComments에서는 게시물 ID(또는 댓글이 연결되는 페이지)를 URL ID라고 부릅니다. 이는 URL이거나 ID일 수 있기 때문입니다.
다음과 같이 URL ID를 정의하세요. 컴포넌트는 config 객체의 변경을 감시하여 다시 로드하므로 URL ID를 업데이트할 수 있습니다.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Account Region (ATTENTION: EU Customers)

계정이 EU에 있는 경우 위젯 구성에서 `region = 'eu'`로 설정하세요. 예:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

그렇지 않으면 `region`을 정의할 필요가 없습니다.