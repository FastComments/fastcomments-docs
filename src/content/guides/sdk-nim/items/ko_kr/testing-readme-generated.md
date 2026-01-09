필수 환경 변수를 설정하세요:

```bash
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"
```

테스트를 실행하세요:

```bash
nimble test
```

또는 특정 테스트를 실행하세요:

```bash
nim c -r tests/test_sso.nim
nim c -r tests/test_sso_integration.nim
```