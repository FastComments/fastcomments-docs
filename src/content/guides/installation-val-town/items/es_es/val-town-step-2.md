`tenantId: "demo"` es un sandbox público compartido. Funciona sin registro, por eso los ejemplos lo usan, pero todos los demás que prueban FastComments escriben en los mismos hilos y cualquiera puede moderarlos. Cambia antes de publicar cualquier cosa que te importe.

Tu ID de inquilino está en la [página de secretos de API](https://fastcomments.com/auth/my-account/api-secret).

Un ID de inquilino es público y pertenece al código del navegador. Un secreto de API no lo es, y nada en esta página necesita uno.

## Léelo desde una variable de entorno

Los vals de Val Town son públicos en el nivel gratuito, por lo que su origen es legible por todos. Mantén cualquier cosa sensible en variables de entorno, leídas con `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Esto es más importante de lo habitual en Val Town por una segunda razón: **remixar un val copia las claves de variables de entorno, pero no sus valores.** Un secreto guardado en una variable de entorno no sigue a tu val en la cuenta de otra persona. Un secreto escrito en un archivo sí lo hace.

Revertir a `"demo"` mantiene el val funcionando para cualquiera que lo remixe antes de establecer su propio inquilino.

## Cuentas EU

Una cuenta, sus datos y sus claves viven en una sola región. Si la tuya se creó en `eu.fastcomments.com`, cada configuración de widget también necesita `region: "eu"`, y los scripts se cargan desde `cdn-eu.fastcomments.com`. De lo contrario, déjalos tal cual.