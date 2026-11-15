Una factura Flex es una lista de conceptos, uno por tipo de uso, cada uno mostrando la cantidad y la tarifa, por ejemplo `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Las tarifas están en la [página de precios](https://fastcomments.com/traffic-pricing) y en tu página de [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Esta sección explica qué significa cada cantidad.

### Contados Durante El Mes

Estos se reinician cada mes y se cuentan desde el primer hasta el último día del mes calendario (UTC):

- **Page Loads**: el número de veces que un widget o feed de comentarios se cargó en tus páginas. Cada carga del widget de comentarios cuenta, sin importar cuántos comentarios muestre. No hay cobro por hilo de comentarios ni por página.
- **API Credits**: llamadas a la API REST con tu clave de API.
- **Small Widgets Credits**: cargas de los widgets pequeños, como contadores de comentarios y badges.
- **LLM Tokens**: tokens utilizados por [AI Agents](/guide-ai-agents.html) y otras funciones de LLM.

### Contados Al Final Del Mes

Estos son una instantánea tomada cuando se crea la factura:

- **Comments Stored**: el número total de comentarios en tu cuenta, no solo los añadidos este mes. Eliminar comentarios lo reduce, excepto que un comentario eliminado con respuestas se mantiene como marcador de posición para que las respuestas sigan teniendo sentido.
- **SSO Users**: el número total de usuarios SSO que tu sitio ha creado, incluidas personas que comentaron una vez hace mucho tiempo. Si esta es la mayor parte de tu factura, pregunta al soporte sobre la facturación por usuarios activos mensuales, donde solo cuentan los usuarios que iniciaron sesión durante el mes.
- **Moderators**, **Admins**, **SSO Moderators** y **SSO Admins**: asientos, cubiertos en [Admin and Moderator Seats](#admin-and-moderator-seats). El primer administrador es gratuito.
- **Domains**: los sitios en tu cuenta. El primer sitio es gratuito y cada uno adicional se factura por mes, mientras permanezca en la cuenta. Un subdominio solo cuenta si lo añades como sitio propio; un dominio de nivel superior cubre sus subdominios. Ver [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: subcuentas gestionadas por esta cuenta, si las hay, más su propio uso como líneas separadas.

### Mínimo

Si las líneas suman menos que el mínimo Flex, la factura se ajusta hasta el mínimo. Un sitio pequeño con un administrador y un dominio paga solo el mínimo.

### Vista Previa De La Factura

La página de [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) muestra cada línea de la próxima factura con las cifras de hoy, así que no hay sorpresas. Los conteos de cargas de página, API y widgets pequeños pueden tener hasta un minuto de retraso; el resto es en vivo.

### Otras Monedas

El uso de Flex se cotiza en dólares estadounidenses. Si la moneda de tu factura es otra, cada línea se convierte al tipo de cambio del día en que se crea la factura, por lo que el mismo uso puede costar cantidades ligeramente diferentes de un mes a otro. Ver [Currencies](#currencies).