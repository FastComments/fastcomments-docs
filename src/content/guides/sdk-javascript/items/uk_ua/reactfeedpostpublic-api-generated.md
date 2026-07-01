## Параметри

| Назва | Тип | Обовʼязково | Опис |
|------|------|------------|------|
| tenantId | string | Так |  |
| postId | string | Так |  |
| reactBodyParams | ReactBodyParams | Так |  |
| isUndo | boolean | Ні |  |
| broadcastId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ReactFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublicResponse.ts)

## Приклад

[inline-code-attrs-start title = 'reactFeedPostPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const postId: string = 'post_98765';
  const reactBodyParams: ReactBodyParams = { reaction: 'thumbs_up' };
  const isUndo: boolean = false;
  const broadcastId: string = 'broadcast_abcde';
  const sso: string = 'sso_token_xyz';

  const response: ReactFeedPostPublicResponse = await reactFeedPostPublic(
    tenantId,
    postId,
    reactBodyParams,
    isUndo,
    broadcastId,
    sso
  );

  console.log(response);
})();
[inline-code-end]