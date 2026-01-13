---
Θα δείτε ότι πρέπει να περάσετε ένα `broadcastId` σε ορισμένες κλήσεις API. Όταν λαμβάνετε συμβάντα, θα λάβετε πίσω αυτό το ID, ώστε να γνωρίζετε να αγνοήσετε το συμβάν εάν σκοπεύετε να εφαρμόσετε αλλαγές με αισιοδοξία στον πελάτη (κάτι που πιθανόν θα θελήσετε να κάνετε, καθώς προσφέρει την καλύτερη εμπειρία). Περάστε εδώ ένα UUID. Το ID πρέπει να είναι αρκετά μοναδικό ώστε να μην εμφανιστεί δύο φορές σε μια συνεδρία προγράμματος περιήγησης.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Μοναδικό ID (UUID) για αυτήν την ενέργεια
  }
});
```
---