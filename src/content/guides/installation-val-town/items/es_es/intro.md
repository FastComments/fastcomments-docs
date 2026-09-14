[Val Town](https://val.town) ejecuta TypeScript en Deno, por lo que un val es un servidor real. Eso lo hace adecuado para FastComments: el widget es una etiqueta script en la página, y cualquier cosa que necesite un secreto, como Secure SSO o la verificación de un webhook, puede ejecutarse del lado del servidor en el mismo val.

Esta guía cubre cómo agregar el widget de comentarios a un val HTTP, mostrar recuentos de comentarios en una página de índice, iniciar sesión de usuarios con la cuenta de Val Town que ya poseen, y recibir webhooks de comentarios.

No necesitas una cuenta para probarlo. Los ejemplos usan `tenantId: "demo"`, un sandbox compartido, y el Paso 2 cubre cómo cambiar a tu propio.