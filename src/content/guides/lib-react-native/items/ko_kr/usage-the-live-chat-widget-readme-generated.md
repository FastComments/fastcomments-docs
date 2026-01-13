```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // 귀하의 테넌트 ID입니다. https://fastcomments.com/auth/my-account/api-secret에서 가져올 수 있습니다
  const myAppPageId = 'native-test'; // 앱 내 댓글 스레드의 ID 또는 URL입니다.
  const myAppPageUrl = 'https://example.com/external-page'; // 외부 페이지의 URL을 선택적으로 설정할 수 있습니다.
  const myAppPageTitle = 'Example Title'; // 그리고 아마도 이 콘텐츠의 제목을 지정하고 싶을 것입니다.
  const config = {
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  };

  return (
      <FastCommentsLiveChatWidget config={config}/>
  );
```