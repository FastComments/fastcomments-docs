### Instalar desde la Tienda de aplicaciones de Shopify

1. Abre el [listado de FastComments en la Tienda de aplicaciones de Shopify](https://apps.shopify.com/fastcomments).
2. Haz clic en **Add app** y elige el plan que quieras durante el flujo de instalación.
3. Shopify te redirige de vuelta al administrador de FastComments dentro de Shopify cuando se completa la instalación.

Esa es toda la instalación. No hay nada que pegar en los archivos de tu tema.

### Qué se configura por ti

La instalación ejecuta todo lo que, de otro modo, harías a mano:

- Se crea un tenant de FastComments para tu tienda y se vincula a tu dominio de tienda.
- La URL de la tienda se agrega a los dominios autorizados del tenant, para que los comentarios se carguen sin error de dominio.
- Se escribe un metafield de tienda `fastcomments.tenant_id` para que cada bloque sepa contra qué tenant debe renderizarse.
- El inicio de sesión único para tus clientes de Shopify se habilita por defecto.
- La facturación se realiza a través de los Precios gestionados por Shopify. Los cargos aparecen en tu factura habitual de Shopify. Actualiza, degrada o cancela desde **Configuración > Aplicaciones y canales de venta > FastComments** en tu administrador de Shopify.

Si tu tienda ya era cliente de FastComments antes de instalar la app, la instalación reutiliza el tenant existente en lugar de crear uno nuevo.

### El administrador integrado

Cuando abres la app de FastComments desde tu administrador de Shopify, llegas a un panel con mosaicos de un solo clic hacia el backend completo de FastComments:

- **Panel**: ajustes de cuenta, uso y detalles de suscripción.
- **Cola de moderación**: aprobar, rechazar y responder comentarios en toda tu tienda.
- **Personalizar**: ajustar colores del widget, fuentes, reglas de moderación y configuración.
- **Asistente de calificaciones y reseñas**: configurar valoraciones con estrellas y preguntas de reseña si deseas usar el bloque Resumen de reseñas.

Cada mosaico abre FastComments con un enlace de inicio de sesión de un solo uso, por lo que no necesitas una sesión separada.

### Siguiente: agrega bloques a tu tienda

Abre el editor de temas de Shopify (**Tienda online > Temas > Personalizar**), abre la plantilla a la que quieres añadir comentarios o reseñas, y haz clic en **Add block**. Los bloques de FastComments aparecen bajo **Aplicaciones**. El resto de esta guía cubre cada uno.