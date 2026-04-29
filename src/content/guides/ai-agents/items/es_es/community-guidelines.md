El **Directrices comunitarias** campo en el formulario de edición es un bloque de texto de política opcional incluido en el mensaje de contexto con rol de usuario en cada ejecución para este agente. Está enmarcado como texto no confiable (el mismo enmarcado que la plataforma aplica a los cuerpos de comentarios y a otro contenido suministrado por el usuario), por lo que el modelo lo trata como referencia de política, no como instrucciones del sistema. Es el lugar canónico para escribir "qué comportamiento está y no está permitido en este sitio" para que el agente lo aplique de forma coherente.

### Cómo difiere del prompt inicial

- **Prompt inicial** - el rol del agente y su estilo de toma de decisiones. "Eres un moderador. Prefiere advertir antes que expulsar."
- **Directrices comunitarias** - las reglas de tu comunidad, en lenguaje de política. "No ataques personales. No enlaces promocionales desde cuentas con menos de 24 horas. Los comentarios fuera de tema pueden eliminarse si un hilo está caldeado."

Ambos fluyen en la misma ventana de contexto, pero entran en diferentes capas: el prompt inicial forma parte del rol de sistema, el documento de directrices está enmarcado como texto dentro del mensaje de contexto con rol de usuario. La separación facilita la edición cuando quieres actualizar uno sin re-leer el otro.

### Qué es un buen documento de directrices

Un documento breve, específico, escrito por un humano. Las listas funcionan mejor que la prosa:

[inline-code-attrs-start title = 'Ejemplo de directrices comunitarias'; type='text' inline-code-attrs-end]
[inline-code-start]
Permitido:
- Desacuerdos sustantivos, incluso en tono fuerte.
- Enlaces a fuentes originales, incluso desde cuentas nuevas.
- Comentarios fuera de tema si el hilo principal lo permite.

No permitidos:
- Ataques personales contra usuarios nombrados específicamente.
- Doxxing o divulgación de información privada.
- Actividad promocional coordinada (múltiples comentarios promoviendo el mismo enlace externo).
- Comentarios que existen únicamente para descarrilar la discusión.

Casos límite:
- Lenguaje fuerte sin objetivo. Permitido si no está dirigido a una persona.
- Temas políticos fuera del tema de la página. Fuera de tema; advertir primero, no eliminar a menos que sean persistentes.
[inline-code-end]

El agente aplica esto en cada ejecución. Si cambias las directrices, el cambio entra en vigor en el siguiente desencadenante; las ejecuciones pasadas no se reevalúan retroactivamente.

### Qué no poner aquí

- **Instrucciones de formato de salida** ("responder en HTML", "usar emoji"). Esas pertenecen al [prompt inicial](#personality-prompt).
- **Texto localizado.** El documento de directrices, como el prompt, es **solo en inglés** por la misma razón: la traducción automática puede cambiar el comportamiento del agente de forma silenciosa. Si tienes políticas que varían por localidad, escríbelas todas en inglés en este mismo documento y estructura el documento como "para páginas en alemán: ..."
- **Citas largas de políticas externas.** Parafrasea. Un contexto largo cuesta tokens en cada ejecución.
- **PII o secretos.** Este texto se envía al proveedor de LLM en cada ejecución.

### Longitud

El campo está limitado a **4000 caracteres** (impuesto tanto por el formulario como por la ruta de guardado). El coste en tokens en cada ejecución es proporcional a la longitud, así que incluso dentro del límite unas pocas centenas de palabras suelen ser suficientes. Si las políticas de tu comunidad abarcan muchas páginas, resume las partes que el agente necesita en un breve específico para este campo.

### Versionado

No hay historial de versiones integrado para el documento de directrices: el último valor guardado es el que usa el agente. Si quieres historial, copia el documento en tu propio sistema de seguimiento antes de cada edición importante. El flujo [Refinar indicaciones](#refining-prompts) puede registrar cambios al *prompt inicial* pero no versiona el documento de directrices.

---