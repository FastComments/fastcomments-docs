Ви побачите, що потрібно передавати `broadcastId` у деяких викликах API. Коли ви отримуватимете події, ви отримаєте назад цей ID, тож знатимете, що ігнорувати подію, якщо плануєте оптимістично застосовувати зміни на клієнті (а саме так, ймовірно, і варто робити, оскільки це забезпечує найкращий досвід). Передайте сюди UUID. ID повинен бути достатньо унікальним, щоб не повторюватися двічі в одній сесії браузера.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Унікальний ідентифікатор для цієї операції
  }
});
```