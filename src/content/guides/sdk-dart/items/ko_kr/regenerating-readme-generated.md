제너레이터는 로컬에서 실행 중인 FastComments 서버  
(`http://localhost:3001/js/swagger.json`)를 사용할 수 있을 때 사양을 가져오고, 그렇지 않으면  
커밋된 `openapi.json`을 사용합니다.

```bash
python3 update.py
```

`node`/`npx`(`@openapitools/openapi-generator-cli`용)와 Java가 필요합니다.