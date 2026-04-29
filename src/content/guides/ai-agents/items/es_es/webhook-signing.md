Cada webhook de agente está firmado con HMAC-SHA256 usando el API secret de tu tenant. El mismo esquema de firma se usa para los comment webhooks de FastComments: si ya los has integrado, los agent webhooks reutilizan la misma cabecera de firma y el mismo flujo de verificación.

### Why signing

Sin una firma, un atacante que conozca la URL de tu webhook podría hacer POST con eventos falsificados que parecen venir de FastComments. Firmar significa que tu endpoint puede verificar que cada entrega es auténtica antes de actuar sobre ella.

### How signatures work

Para cada entrega:

1. La plataforma busca el API secret para el tenant + dominio coincidente (ver [Webhooks Overview](#webhooks-overview)).
2. Emite la marca de tiempo Unix actual (en milisegundos) en la cabecera `X-FastComments-Timestamp`.
3. Calcula `HMAC-SHA256(api_secret, "${timestamp}.${raw_request_body}")` (estilo Stripe) y emite el resultado como `sha256=<hex>` en la cabecera `X-FastComments-Signature`.
4. Tu endpoint lee la cabecera de timestamp, recalcula el HMAC sobre `${timestamp}.${body}` que recibió, lo compara con el valor `sha256=<hex>` en la cabecera de firma, y rechaza las discrepancias.

El cuerpo que se firma son los **bytes exactos** que la plataforma envió, con el prefijo `${timestamp}.` — tu verificador debe usar el cuerpo de la solicitud sin procesar (raw request body), no una cadena JSON re-serializada (el orden de las claves y los espacios en blanco serían distintos).

### API secret

El mismo API Secret usado por los [comment webhooks](/guide-webhooks.html). Es por (tenant, domain) y se gestiona en los ajustes de API de tu tenant. Si rotas el secret, deberías desplegar de nuevo tu verificador para leer el nuevo valor antes de la siguiente entrega.

Cuando la plataforma no encuentra **ningún API secret** para el dominio coincidente, la entrega no se realiza. El registro del webhook anota la falla con la razón "no API secret".

### Verification example (Node.js)

[inline-code-attrs-start title = 'Ejemplo de verificación de firma del webhook'; type='javascript' inline-code-attrs-end]
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

Usa `timingSafeEqual` en lugar de `===` para evitar fugas por canales temporales de la firma.

### What's in the signed body

El sobre completo más el bloque `data` específico del evento. Ver [Webhook Payloads](#webhook-payloads).

### Recommendations

- **Verifica en cada entrega.** Si tu endpoint acepta solicitudes no firmadas, no tienes garantía de integridad.
- **Rechaza en caso de desacuerdo de firma.** Devuelve 401 o 403; no respondas 200 OK con una firma incorrecta, o enmascararás ataques en los registros de entrega.
- **Usa HTTPS.** Las firmas protegen la integridad; TLS protege la confidencialidad (tanto tu secret como el texto del comentario en la carga útil).
- **Rota los secrets** cuando miembros del equipo con acceso se vayan, o según un calendario.

### Replay protection

La firma por sí sola no previene ataques de reproducción: un atacante que capture una entrega firmada real puede reenviarla. La protección contra replays corresponde a tu endpoint:

- Usa el campo del sobre `occurredAt` y rechaza entregas más antiguas que, por ejemplo, 5 minutos.
- Usa `triggerId` o `approvalId` como clave de deduplicación — si ya lo has procesado, ignora el duplicado.

### See also

- [Webhooks Overview](#webhooks-overview).
- [Webhook Payloads](#webhook-payloads).
- La guía principal de [Webhooks](/guide-webhooks.html) para la infraestructura de firma en general.