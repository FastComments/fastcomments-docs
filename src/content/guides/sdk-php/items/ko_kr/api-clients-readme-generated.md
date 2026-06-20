SDK는 세 가지 API 클라이언트 클래스를 제공합니다:

- **`DefaultApi`** — 서버 측 사용을 위한 API 키 인증 메서드입니다. API 키 구성 방법은 [시작하기](#getting-started-readme-generated)를 참조하세요.
- **`PublicApi`** — API 키가 필요 없는 공개 메서드로, 브라우저 및 모바일 앱에서 호출해도 안전합니다.
- **`ModerationApi`** — 중재자 대시보드를 위한 메서드: 댓글 목록 조회, 집계, 검색, 로깅 및 내보내기; 중재 작업(제거/복원, 신고, 검토/스팸/승인 상태 설정, 투표, 스레드 재개/종료); 차단(댓글 금지, 되돌리기, 사전 차단 요약, 차단 상태 및 환경설정, 차단된 사용자 수); 그리고 배지 및 신뢰(배지 부여/제거, 수동 배지, 신뢰도 조회/설정, 사용자 내부 프로필). 모든 `ModerationApi` 메서드는 동작하는 중재자를 SSO로 인증하기 위해 `$sso` 매개변수를 허용합니다.

### PublicApi 사용하기

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 공개 메서드는 API 키를 필요로 하지 않습니다.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$url_id = 'url_id_example'; // 문자열

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### ModerationApi 사용하기

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // 문자열 - 중재자를 SSO로 인증하는 페이로드

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```