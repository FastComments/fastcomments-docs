### 개요

FastComments Image Chat은 표준 FastComments 댓글 위젯을 확장한 것으로, 기본 위젯의 모든 구성 옵션을 상속하면서 이미지 주석에 특화된 몇 가지 옵션을 추가합니다.

### 필수 구성

#### tenantId

FastComments 테넌트 ID가 필요합니다. [FastComments 대시보드](https://fastcomments.com/auth/my-account/api-secret)에서 확인할 수 있습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat 전용 옵션

#### urlId

기본적으로 Image Chat은 페이지 URL, 이미지 소스, X/Y 좌표를 기반으로 각 대화에 대한 고유 식별자를 생성합니다. 사용자 지정 `urlId`로 이를 재정의할 수 있습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

URL 구조가 변경될 수 있지만 동일한 대화를 유지하고 싶거나, 여러 페이지에서 주석을 공유하고 싶을 때 유용합니다.

#### chatSquarePercentage

클릭 가능한 채팅 마커의 크기를 이미지 너비의 백분율로 제어합니다. 기본값은 5%이며, 각 마커가 이미지 너비의 5%임을 의미합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // 더 크고 눈에 잘 띄는 마커
});
```

값이 작을수록 방해가 덜 되는 마커가 생성되어 세밀한 이미지에 더 적합합니다. 값이 클수록 마커를 더 쉽게 보고 클릭할 수 있어 복잡한 이미지나 모바일 기기 사용자에게 유용합니다.

#### hasDarkBackground

페이지에 어두운 배경이 있을 때 다크 모드 스타일을 활성화합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

댓글 수가 변경될 때마다 실행되는 콜백 함수입니다. 배지나 페이지 제목 같은 UI 요소를 업데이트하는 데 유용합니다.

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

Image Chat은 표준 댓글 위젯을 확장하므로 기본 FastComments 위젯의 모든 구성 옵션을 사용할 수 있습니다. 자주 사용되는 옵션은 다음과 같습니다:

#### locale

위젯 UI의 언어를 설정합니다. FastComments는 수십 개의 언어를 지원합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // 스페인어
});
```

#### readonly

모든 대화를 읽기 전용으로 만듭니다. 사용자는 기존 마커와 토론을 볼 수 있지만 새로 만들거나 답장할 수는 없습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso 및 simpleSSO

Single Sign-On을 사용하여 인증 시스템과 통합합니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO 구성
    }
});
```

인증 옵션에 대한 전체 세부 사항은 SSO 문서를 참조하세요.

#### maxReplyDepth

답글이 몇 단계까지 중첩될 수 있는지 제어합니다. 기본적으로 Image Chat은 이 값을 0으로 설정하여 모든 댓글이 평면적(중첩된 답글 없음)이 되도록 합니다. 스레드 형태의 대화를 원한다면 이 값을 변경할 수 있습니다.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3단계 중첩 허용
});
```

### 내부 구성

이 옵션들은 Image Chat에 의해 자동으로 설정되며 재정의해서는 안 됩니다:

`productId`는 Image Chat을 위해 자동으로 `2`로 설정됩니다. 채팅 창 기능을 제공하기 위해 `floating-chat` 확장이 자동으로 로드됩니다. 위젯은 모바일 기기(너비 768px 미만 화면)를 자동으로 감지하고 전체 화면 채팅 창으로 그에 맞게 UI를 조정합니다.

### 대상 요소 유연성

`FastCommentsImageChat`의 첫 번째 매개변수는 `<img>` 요소를 직접 전달하거나 이미지가 포함된 컨테이너 요소를 전달할 수 있습니다:

```javascript
// 이미지 요소 직접 전달
FastCommentsImageChat(document.getElementById('my-image'), config);

// 이미지가 포함된 컨테이너 전달
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

컨테이너 요소를 전달하면 위젯이 이미지를 자동으로 찾습니다.

### 전체 예제

여러 구성 옵션을 함께 사용하는 예제입니다:

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

기본 위젯에서 상속된 사용 가능한 모든 구성 옵션의 전체 목록은 FastComments 기본 구성 문서를 참조하세요.