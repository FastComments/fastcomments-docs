Видећете да треба да проследите `broadcastId` у неким API позивима. Када примите догађаје, добићете овај ID назад, па ћете знати да игноришете догађај ако планирате да оптимистички примените промене на клијенту (што ћете вероватно желети, јер пружа најбоље корисничко искуство). Проследите UUID овде. ID треба да буде довољно јединствен да се не појави два пута у прегледачкој сесији.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Јединствени ID за ову операцију
  }
});
```