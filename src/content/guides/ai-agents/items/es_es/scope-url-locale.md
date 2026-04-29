Por defecto, un agente se ejecuta en todo tu tenant: cada página, cada locale. Las secciones **Alcance** y **Locales** en el formulario de edición te permiten limitar eso.

### Restringir a páginas específicas

El campo **Restrict to specific pages** acepta un patrón de URL por línea, en la sintaxis url-pattern glob. El agente solo se ejecuta en comentarios cuyo URL de página coincida con al menos uno de los patrones. Ejemplos:

- `/news/*` - cualquier página bajo `/news`.
- `/forums/*` - cualquier página bajo `/forums`.
- `/blog/2026/*` - cualquier página bajo `/blog/2026`.
- (múltiples líneas juntas) - el agente se ejecuta si **cualquiera** de los patrones coincide.

Máximo: 50 patrones por agente. Los patrones deben ser url-pattern globs válidos: el formulario rechaza los malformados con un error específico.

Cuando el campo está **vacío**, el agente se ejecuta en todas las páginas del tenant.

Cuando el campo está **no vacío**, el agente falla cerrado: cualquier disparador cuyo comentario no tenga `urlId` (p. ej. eventos a nivel tenant sin contexto de página) se omite. Esto es por diseño: "scoped to /news/*" no debería silenciosamente pasar a "everything".

### Restringir a locales específicos

El selector dual **Restrict to specific locales** acepta IDs de locale de FastComments (`en_us`, `zh_cn`, `de_de`, etc.). El agente solo se ejecuta en comentarios cuyo locale detectado esté en la lista seleccionada.

El locale detectado proviene del campo `locale` del comentario, que es establecido por el widget de comentarios en el momento de publicar, basado en el locale de la página.

Cuando **no se seleccionan locales**, el agente se ejecuta en todos los locales.

Cuando **se selecciona uno o más locales**, el agente falla cerrado: se omiten los disparadores sin comentario, o los disparadores sobre comentarios sin el campo `locale`.

### Ámbito combinado

Los filtros de URL y de locale se combinan con un AND. Un disparador solo activa el agente si **ambos** filtros lo permiten.

Patrones útiles:
- `/news/*` patrón de URL + `en_us` locale - solo la sección de noticias en inglés.
- Sin filtro de URL + varios locales - a nivel tenant, pero solo para los idiomas para los que se escribió el prompt de este agente.

### Por qué limitar el alcance de un agente

- **Costo.** Limitar el ámbito reduce el volumen de disparadores que el agente tiene que procesar y, por ende, reduce el gasto en tokens.
- **Corrección.** Un prompt de resumen ajustado para artículos técnicos puede producir resultados pobres en páginas de producto. Limitar el ámbito es una herramienta más precisa que pedir al prompt que "ignore páginas no técnicas" en inglés.
- **Comportamiento específico por locale.** Un saludo de bienvenida que solo escribe en alemán debería ejecutarse únicamente en comentarios con locale alemán. Combina el alcance `de_de` con un tono en alemán en el [mensaje inicial](#personality-prompt).

### Qué no hace el alcance

- No cambia el **agent slot count** (ver [Planes y elegibilidad](#plans-and-eligibility)) - un agente con ámbito sigue ocupando un slot.
- No cambia los [Límites de presupuesto](#budgets-overview) - los límites diarios y mensuales por agente se aplican a través de todos los disparadores coincidentes.
- No aplica el ámbito de forma retroactiva a ejecuciones pasadas: el historial de ejecuciones muestra todo lo que el agente hizo, incluso si después restringes su ámbito.