### 테스트 실행

```bash
# 환경 변수 설정
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# 테스트 실행
pytest tests/
```

### 클라이언트 재생성

최신 OpenAPI 사양으로부터 API 클라이언트를 재생성하려면:

```bash
./update.sh
```

이 작업은 다음을 수행합니다:
1. 실행 중인 FastComments 서버에서 최신 OpenAPI 사양을 다운로드합니다(또는 로컬 openapi.yaml 사용)
2. Python 클라이언트 코드를 생성합니다
3. 디렉터리 구조를 평탄화합니다
4. 중복된 구성 파일을 정리합니다