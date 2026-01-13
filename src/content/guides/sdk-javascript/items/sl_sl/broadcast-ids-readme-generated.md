Videli boste, da naj bi pri nekaterih klicih API posredovali `broadcastId`. Ko prejmete dogodke, boste to ID prejeli nazaj, tako da boste vedeli, da dogodek prezrete, če nameravate optimistično uporabiti spremembe na odjemalcu (kar boste verjetno želeli storiti, saj ponuja najboljšo izkušnjo). Tukaj posredujte UUID. ID naj bo dovolj edinstven, da se v isti brskalni seji ne pojavi dvakrat.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // Edinstven ID za to operacijo
  }
});
```