Zobaczysz, że w niektórych wywołaniach API należy przekazać `broadcastId`. Gdy otrzymasz zdarzenia, otrzymasz ten identyfikator z powrotem, dzięki czemu będziesz wiedzieć, że możesz zignorować zdarzenie, jeśli planujesz optimistycznie zastosować zmiany po stronie klienta (co prawdopodobnie będziesz chciał zrobić, ponieważ zapewnia najlepsze doświadczenie). Przekaż tutaj UUID. Identyfikator powinien być wystarczająco unikalny, aby nie pojawił się dwa razy w sesji przeglądarki.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Unikalny identyfikator dla tej operacji
  }
});
```