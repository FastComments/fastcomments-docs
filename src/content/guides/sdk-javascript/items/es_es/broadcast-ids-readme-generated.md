Verás que debes pasar un `broadcastId` en algunas llamadas a la API. Cuando recibas eventos, obtendrás este ID de vuelta, de modo que sepas ignorar el evento si planeas aplicar los cambios de forma optimista en el cliente (lo cual probablemente querrás hacer, ya que ofrece la mejor experiencia). Pasa un UUID aquí. El ID debe ser lo suficientemente único como para no ocurrir dos veces en una sesión del navegador.

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // ID único para esta operación
  }
});
```