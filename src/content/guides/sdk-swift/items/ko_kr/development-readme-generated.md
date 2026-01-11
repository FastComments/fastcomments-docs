### 클라이언트 재생성

최신 OpenAPI 명세에서 API 클라이언트를 다시 생성하려면:

1. FastComments 서버가 로컬에서 `http://localhost:3001`에 실행 중인지 확인하세요
2. 업데이트 스크립트를 실행하세요:

```bash
./update.sh
```

이 작업은 다음을 수행합니다:
- 최신 OpenAPI 명세를 다운로드합니다
- Swift 클라이언트 코드를 생성합니다 (API 문서는 client/docs에 있습니다)
- 모든 것이 정상 작동하는지 확인하기 위해 패키지를 빌드합니다