Видећете да треба да проследите `broadcastId` у неким API позивима. Када примате догађаје, добићете овај ИД назад, тако да знате да игноришете догађај ако планирате оптимистички да примените промене на клијенту (што ћете вероватно желети да урадите јер пружа најбоље искуство). Проследите UUID овде. ИД треба да буде довољно јединствен да се не појави два пута у сесији прегледача.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Јединствени ИД за ову операцију
  }
});
```