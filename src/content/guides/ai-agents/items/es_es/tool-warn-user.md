La herramienta Warn envía un aviso privado por DM a un usuario sobre un comentario específico, y al mismo tiempo registra la advertencia en la [memoria del agente](#agent-memory-system) compartida. Las dos escrituras son atómicas: el usuario nunca ve una advertencia que no esté también registrada.

### Por qué existe

La política de escalada de la plataforma es **advertir primero, banear solo si el usuario reincide**. La herramienta Warn es lo que hace que esa política sea aplicable: le da al usuario la oportunidad de corregirse, y el registro de la advertencia es lo que un agente futuro encuentra cuando busca en la memoria antes de considerar un baneo.

La herramienta también desduplica: si el agente ya ha emitido una advertencia al mismo usuario sobre el mismo comentario, una segunda advertencia no tiene efecto. Así, un LLM que entre en bucle o vuelva a activarse sobre el mismo comentario no puede spamear al usuario con múltiples advertencias.

### Qué incluir en la advertencia

Un mensaje breve (limitado a 1000 caracteres) mostrado al usuario como un DM. Las advertencias fuertes son:

- **Específicas** - "Los ataques personales a usuarios nombrados no están permitidos en esta comunidad" es mejor que "tu comentario fue marcado."
- **Breves** - como máximo unas pocas frases.
- **Accionables** - dile al usuario qué cambiar. "Por favor edita tu comentario para eliminar al usuario nombrado, o será eliminado."

Tú no redactas el mensaje; lo hace el agente, basándose en el [prompt inicial](#personality-prompt) y las [directrices de la comunidad](#community-guidelines). Tu trabajo es escribir un prompt que genere buenas advertencias.

### Cuándo permitirlo

Para cualquier agente de tipo moderación. La plantilla Moderator lo habilita por defecto.

### Aprobaciones

Menos frecuentemente sometido a aprobación que [Ban user](#tool-ban-user). Merece la pena someterlo a aprobación durante las primeras semanas de vida del agente para que puedas detectar advertencias malas antes de que se envíen, pero la mayoría de operadores elimina el requisito una vez que el agente produce salidas fiables.

### Véase también

- [Ban user](#tool-ban-user) - el siguiente paso en la escalada.
- [Sistema de memoria del agente](#agent-memory-system) - donde se almacenan los registros de advertencias.

---