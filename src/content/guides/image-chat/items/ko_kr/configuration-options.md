### 개요

FastComments Image Chat는 표준 FastComments 댓글 위젯을 확장하여 기본 위젯의 모든 구성 옵션을 상속받으면서 이미지 주석에 특화된 몇 가지 옵션을 추가합니다.

### 필수 구성

#### tenantId

FastComments 테넌트 ID가 필요합니다. [FastComments 대시보드](https://fastcomments.com/auth/my-account/api-secret)에서 찾을 수 있습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat 전용 옵션

#### urlId

기본적으로 Image Chat은 페이지 URL, 이미지 소스 및 X/Y 좌표를 기반으로 각 대화에 대한 고유 식별자를 생성합니다. 사용자 정의 `urlId`로 이를 재정의할 수 있습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

URL 구조가 변경될 수 있지만 동일한 대화를 유지하려는 경우나 여러 페이지에서 주석을 공유하려는 경우에 유용합니다.

#### chatSquarePercentage

클릭 가능한 채팅 마커의 크기를 이미지 너비의 백분율로 제어합니다. 기본값은 5%이며, 각 마커가 이미지 너비의 5%를 의미합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // 더 크고 더 잘 보이는 마커
});
```

작은 값은 덜 눈에 띄는 마커를 만들어 상세한 이미지에 더 적합합니다. 큰 값은 복잡한 이미지나 모바일 사용자가 볼 때 마커를 더 쉽게 보고 클릭할 수 있도록 합니다.

#### hasDarkBackground

페이지에 어두운 배경이 있는 경우 다크 모드 스타일을 활성화합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

댓글 수가 변경될 때마다 호출되는 콜백 함수입니다. 배지나 페이지 제목과 같은 UI 요소를 업데이트하는 데 유용합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### 상속된 구성 옵션

Image Chat이 표준 댓글 위젯을 확장하므로 기본 FastComments 위젯의 모든 구성 옵션을 사용할 수 있습니다. 다음은 자주 사용하는 몇 가지 옵션입니다:

#### locale

위젯 UI의 언어를 설정합니다. FastComments는 수십 개의 언어를 지원합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // 스페인어
});
```

#### readonly

모든 대화를 읽기 전용으로 만듭니다. 사용자는 기존 마커와 토론을 볼 수는 있지만 새로 생성하거나 답글을 달 수 없습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

싱글 사인온(SSO)을 사용해 인증 시스템과 통합합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO 구성
    }
});
```

인증 옵션에 대한 자세한 내용은 SSO 문서를 참조하세요.

#### maxReplyDepth

답글의 중첩 깊이를 제어합니다. 기본적으로 Image Chat은 이를 0으로 설정하여 모든 댓글이 평면(중첩 답글 없음)입니다. 스레드형 대화를 원하면 이 값을 변경할 수 있습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 중첩을 3단계 허용
});
```

### 내부 구성

다음 옵션은 Image Chat에 의해 자동으로 설정되며 재정의해서는 안 됩니다:

`productId`는 Image Chat용으로 자동으로 `2`로 설정됩니다. 채팅 창 기능을 제공하기 위해 `floating-chat` 확장이 자동으로 로드됩니다. 위젯은 모바일 기기(폭이 768px 미만인 화면)를 자동으로 감지하여 전체화면 채팅 창으로 UI를 조정합니다.

### 대상 요소 유연성

`FastCommentsImageChat`의 첫 번째 매개변수는 직접적인 `<img>` 요소이거나 그 안에 이미지가 있는 컨테이너 요소일 수 있습니다:

```javascript
// 직접 이미지 요소
FastCommentsImageChat(document.getElementById('my-image'), config);

// 이미지가 포함된 컨테이너
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

컨테이너 요소를 전달하면 위젯이 자동으로 이미지를 찾아줍니다.

### 전체 예제

다음은 여러 구성 옵션을 함께 보여주는 예제입니다:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // SSO 구성
    },
    maxReplyDepth: 1
});
```

기본 위젯에서 상속된 모든 사용 가능한 구성 옵션의 전체 목록은 FastComments 구성 문서를 참조하세요.