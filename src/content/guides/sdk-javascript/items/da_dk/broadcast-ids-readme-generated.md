Du vil se, at du skal sende et `broadcastId` i nogle API-kald. Når du modtager hændelser, får du dette ID tilbage, så du ved, at du kan ignorere eventet, hvis du planlægger at anvende ændringer optimistisk på klienten (hvilket du sandsynligvis vil gøre, da det giver den bedste oplevelse). Send en UUID her. ID'et bør være tilstrækkeligt unikt til ikke at forekomme to gange i en browsersession.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Unikt ID for denne operation
  }
});
```