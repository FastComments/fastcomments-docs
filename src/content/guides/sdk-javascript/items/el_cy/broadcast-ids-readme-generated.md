Θα δείτε ότι πρέπει να περάσετε ένα `broadcastId` σε κάποιες κλήσεις API. Όταν λάβετε γεγονότα, θα σας επιστραφεί αυτό το ID, ώστε να ξέρετε να αγνοήσετε το γεγονός αν σκοπεύετε να εφαρμόσετε αλλαγές στον client με 'optimistic' τρόπο (πιθανότατα θα θέλετε να το κάνετε, καθώς προσφέρει την καλύτερη εμπειρία). Δώστε εδώ ένα UUID. Το ID πρέπει να είναι αρκετά μοναδικό ώστε να μην εμφανιστεί δύο φορές σε μια συνεδρία περιήγησης.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Μοναδικό ID για αυτήν την ενέργεια
  }
});
```