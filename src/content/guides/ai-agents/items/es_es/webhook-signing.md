Cada webhook de agente se firma con HMAC-SHA256 utilizando el secreto de API de tu tenant. Se utiliza el mismo esquema de firma que para los webhooks de comentarios de FastComments - si ya has integrado esos, los webhooks de agente reutilizan la misma cabecera de firma y el mismo flujo de verificación.

### Por qué la firma

Sin una firma, un atacante que conozca la URL de tu webhook podría enviar mediante POST eventos falsificados que parezcan provenir de FastComments. La firma permite que tu endpoint verifique que cada entrega es auténtica antes de actuar sobre ella.

### Cómo funcionan las firmas

Para cada entrega:

1. La plataforma busca el secreto de API para el tenant + dominio coincidente (ver [Descripción general de Webhooks](#webhooks-overview)).
2. Emite la marca de tiempo Unix actual (en milisegundos) en la cabecera `X-FastComments-Timestamp`.
3. Calcula `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (al estilo de Stripe) y emite el resultado como `sha256=<hex>` en la cabecera `X-FastComments-Signature`.
4. Tu endpoint lee la cabecera de la marca de tiempo, recalcula el HMAC sobre `${timestamp}.${body}` que recibió, lo compara con el valor `sha256=<hex>` en la cabecera de firma y rechaza las discrepancias.

El cuerpo que se firma son los **bytes exactos** que la plataforma envió, con el prefijo `${timestamp}.` - tu verificador debe usar el cuerpo de la solicitud sin procesar, no una cadena JSON re-serializada (de lo contrario, el orden de las claves y los espacios en blanco diferirían).

### Secreto de API

Es el mismo API Secret utilizado por los [webhooks de comentarios](/guide-webhooks.html). Es por (tenant, dominio) y se gestiona en la configuración de API de tu tenant. Si rotas el secreto, debes volver a desplegar tu verificador para leer el nuevo valor antes de la siguiente entrega.

Cuando la plataforma no encuentra **ningún secreto de API** para el dominio coincidente, la entrega no se realiza. El registro del webhook registra el fallo con el motivo "no API secret".

### Ejemplo de verificación (Node.js)

[inline-code-attrs-start title = 'Ejemplo de verificación de firma de webhook'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import crypto from 'crypto';

function verifyAgentWebhook(rawBody, signatureHeader, timestampHeader, secret) {
  const expected = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(`${timestampHeader}.${rawBody}`)
    .digest('hex');
  return crypto.timingSafeEqual(
    Buffer.from(expected),
    Buffer.from(signatureHeader),
  );
}
[inline-code-end]

Usa `timingSafeEqual` en lugar de `===` para evitar fugas de la firma por canales de temporización.

### Qué contiene el cuerpo firmado

El sobre completo más el bloque `data` específico del evento. Ver [Cargas útiles de Webhook](#webhook-payloads).

### Recomendaciones

- **Verifica en cada entrega.** Si tu endpoint acepta solicitudes sin firmar, no tienes ninguna garantía de integridad.
- **Rechaza en caso de discrepancia de firma.** Devuelve 401 o 403; no devuelvas 200 OK con una firma incorrecta, o enmascararás ataques en tus registros de entregas.
- **Usa HTTPS.** Las firmas protegen la integridad; TLS protege la confidencialidad (tanto tu secreto como el texto de los comentarios en la carga útil).
- **Rota los secretos** cuando miembros del equipo con acceso se vayan, o según un calendario.

### Protección contra repetición

La firma por sí sola no previene los ataques de repetición - un atacante que haya capturado una entrega firmada real puede reenviarla. La protección contra repetición depende de tu endpoint:

- Usa el campo del sobre `occurredAt` y rechaza entregas anteriores a, por ejemplo, 5 minutos.
- Usa `triggerId` o `approvalId` como clave de deduplicación - si ya lo has procesado, ignora el duplicado.

### Ver también

- [Descripción general de Webhooks](#webhooks-overview).
- [Cargas útiles de Webhook](#webhook-payloads).
- La [guía principal de Webhooks](/guide-webhooks.html) para la infraestructura de firma más amplia.