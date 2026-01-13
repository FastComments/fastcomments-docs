API는 `fastcomments-react`에 비해 약간 다릅니다. 네이티브에서는 [이 구조](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35)를 따르는 config 객체를 전달합니다.

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // 귀하의 tenant id입니다. https://fastcomments.com/auth/my-account/api-secret 에서 가져올 수 있습니다
  const myAppPageId = 'native-test'; // 앱 내 댓글 스레드의 ID 또는 URL입니다.
  const myAppPageUrl = 'https://example.com/external-page'; // 외부 페이지의 url을 선택적으로 설정할 수 있습니다
  const myAppPageTitle = 'Example Title'; // ... 그리고 아마 이 콘텐츠에 대한 제목을 원할 것입니다
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // setConfig()를 호출하면 현재 페이지를 변경하거나 현재 로그인한 사용자를 변경하는 등의 작업을 할 수 있습니다
  // 자세한 내용은 example/src/App.tsx를 참조하세요

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```