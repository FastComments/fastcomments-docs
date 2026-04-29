La herramienta de baneo es la acción más trascendental que puede ejecutar un agente. Banea a un usuario de tu comunidad, con una duración fija y algunas opciones.

### Qué hace

El agente elige una de seis duraciones:

- Una hora
- Un día
- Una semana
- Un mes
- Seis meses
- Un año

También elige entre un **baneo visible** (el usuario ve un mensaje claro de baneo y puede apelar) y un **baneo en la sombra** (el usuario puede seguir publicando pero su contenido está oculto a otros usuarios). Las instrucciones de la plataforma indican al agente que prefiera baneos visibles para casos de primera vez o limítrofes, y baneos en la sombra para reincidentes claramente maliciosos.

### Las dos sub-opciones destructivas

Dos opciones extra están **ocultas al agente por defecto**. Para habilitar cualquiera de ellas, marca la casilla correspondiente en la sección **Opciones de baneo** del formulario de edición del agente:

- **Permitir eliminar todos los comentarios del usuario.** Cuando está habilitado, el agente puede elegir también eliminar cada comentario que el usuario baneado haya publicado en tu tenant. Reservar para spam claro, doxxing o abuso coordinado donde el contenido existente no tiene valor. **Destructivo e irreversible.**
- **Permitir banear por IP.** Cuando está habilitado, el agente puede elegir también banear la IP desde la que se publicó el comentario. Útil contra evasión de baneo con cuentas alternativas. **Evitar en IPs compartidas** (empresas, escuelas, operadores móviles): usuarios inocentes en la misma red serán bloqueados.

La plataforma también aplica restricciones del lado del servidor: incluso si el agente se comporta de forma maliciosa e intenta invocar la opción, la solicitud se rechaza a menos que te hayas suscrito.

### Política de escalamiento

Antes de banear, la plataforma instruye al agente a:

1. Buscar en la [memoria del agente](#agent-memory-system) avisos previos o notas sobre el usuario.
2. Preferir [advertir](#tool-warn-user) al usuario en lugar de banear en las primeras infracciones.
3. Saltarse el paso de advertencia solo en casos claramente graves (contenido ilegal, doxxing, spam coordinado) — y explicar por qué en su justificación.

Esta política está en las instrucciones del agente, no es una regla estricta del lado del servidor, por lo que se recomienda encarecidamente **restringir los baneos para que requieran aprobación**.

### Región UE: se requiere aprobación humana

En la región de la UE, esta herramienta está **configurada para requerir aprobación** por el Artículo 17 de la Ley de Servicios Digitales (Digital Services Act). Cada baneo realizado por cualquier agente en un tenant de la región de la UE llega a la [bandeja de aprobaciones](#approval-workflow) para revisión humana. Ver [Cumplimiento del Artículo 17 de la DSA de la UE](#eu-dsa-compliance).

### Recomendaciones

- Exigir aprobación en todas partes durante al menos el primer mes.
- Siempre exigir aprobación para la opción **eliminar-todos-los-comentarios** si la habilitas: es irreversible.
- Considerar exigir aprobación para la opción de **baneo por IP** incluso después de que el agente haya ganado confianza: el costo de un baneo por IP en una red compartida no aparece en el historial de ejecuciones del agente.

### Véase también

- [Baneo de usuarios](/guide-moderation.html#banning-users) y [Baneo de usuarios con comodines](/guide-moderation.html#banning-users-wildcards) en la guía de moderación para ver cómo funcionan los baneos en toda la plataforma.
- [Advertir al usuario](#tool-warn-user) - el paso de escalada más suave.