Agent memory es un almacén de pares clave-valor con ámbito por tenant, **compartido**, que cada agente en tu tenant puede leer y escribir. Existe para que los agentes puedan conservar contexto entre ejecuciones.

### Por qué existe la memoria

El contexto de la LLM es por ejecución. Sin memoria, un agente que emite una advertencia a un usuario no tiene forma de recordar esa advertencia la próxima vez que vea al mismo usuario. La política de escalada de la plataforma — "advertir antes de banear" — depende de que el agente pueda encontrar la advertencia previa. La memoria es lo que hace que eso funcione.

### Dos tipos de memoria

- **WARNING** - escrita automáticamente como parte del flujo de [`warn_user`](#tool-warn-user). El agente no escribe registros `WARNING` manualmente; son un efecto secundario de advertir a un usuario.
- **NOTE** - escrita por [`save_memory`](#tools-overview). Contexto de propósito general que el agente quiere que otros agentes futuros conozcan.

La política de escalada busca específicamente registros `WARNING` cuando decide si un baneo está justificado.

### Ámbito por tenant, compartido entre agentes

Todos los agentes en tu tenant comparten **un único pool de memoria**. Una nota guardada por el Agente A es visible para las llamadas `search_memory` del Agente B. Esto es intencional: quieres que las notas de un agente de triaje informen las decisiones de un agente moderador.

`tenantId` es establecido por el executor desde el tenant propio del agente - nunca desde los args de la LLM - por lo que las fugas de memoria entre tenants son imposibles por diseño.

### Qué contiene un registro de memoria

Cada entrada de memoria contiene:

- **Qué agente la escribió**, y cuándo.
- **Sobre quién trata** - el usuario que describe esta memoria. El agente no puede inventarlo; la plataforma lo completa automáticamente a partir de lo que desencadenó al agente.
- **Una señal oculta de cuentas alternas** - la plataforma también registra (privadamente) la huella de IP del comentario de origen, para que búsquedas de memoria futuras puedan sacar a la luz notas sobre otras cuentas que publican desde la misma IP. La huella nunca se muestra al agente ni a la LLM.
- **La nota en sí** - hasta 2000 caracteres de texto libre.
- **Etiquetas** para recuperación - hasta 10 etiquetas cortas.
- **Un tipo** - ya sea una warning o una nota general.
- **Un enlace opcional al comentario** - si la memoria está ligada a un comentario específico.

### Comportamiento de búsqueda

[`search_memory`](#tools-overview) devuelve hasta 25 registros, ordenados de más nuevo a más antiguo, con el ámbito establecido automáticamente a (el usuario que desencadenó) O (otras cuentas en la IP que desencadenó). Los resultados también están limitados a 8000 caracteres en total a través de todo el contenido devuelto: las entradas más antiguas se descartan si se alcanza el límite.

El agente no pasa `userId` ni `targetIpHash`. Ambos son establecidos por el executor.

### Persistencia

La memoria no tiene **TTL**. Los registros persisten hasta que se eliminan explícitamente. Los registros WARNING sobre un usuario intencionalmente nunca se eliminan automáticamente: el historial de escaladas debe poder encontrarse indefinidamente o la comprobación de la plataforma de "buscar antes de banear" no tendría sentido.

Las tres maneras en que se elimina la memoria:

- Un moderador borra el comentario subyacente - cualquier memoria ligada a ese comentario se elimina en cascada.
- Un usuario es eliminado - todas las entradas de memoria sobre ese usuario se eliminan en la misma transacción.
- Tu tenant es eliminado.

Hoy no existe una UI de administrador para eliminar registros de memoria individuales.

### Memoria en dry-run

Los agentes en dry-run no escriben memoria. Esto es por diseño: las decisiones hipotéticas de un agente en dry-run no deben contaminar el pool de memoria compartido. La lectura mediante `search_memory` funciona normalmente en dry-run - el agente puede ver memorias reales de agentes en vivo - simplemente no puede añadir a ellas.

### Memoria en replays

Igual que en dry-run: los agentes en replay no escriben memoria. Los replays son solo de vista previa. Ver [Test Runs (Replays)](#test-runs-replays).

### Resumen de restricciones

| Límite | Valor |
|---|---|
| Longitud máxima del contenido de memoria | 2000 chars |
| Longitud máxima de una etiqueta de memoria | 64 chars |
| Máximo de etiquetas de memoria | 10 |
| Longitud máxima de consulta de memoria | 200 chars |
| Límite de resultados de búsqueda de memoria | 25 records |
| Tope total de contenido en búsqueda de memoria | 8000 chars |

### Véase también

- [Tool: save_memory](#tools-overview) para escribir.
- [Tool: search_memory](#tools-overview) para leer.
- [Tool: warn_user](#tool-warn-user) - la única herramienta que escribe memoria de tipo WARNING.
- [Tool: ban_user](#tool-ban-user) - el prompt del sistema requiere llamar a `search_memory` antes de esto.