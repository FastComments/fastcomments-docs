Un agente tiene uno de tres estados:

### Disabled

El agente está apagado. No se procesan disparadores y el agente no aparece en la ruta de despacho. Su historial de ejecuciones, analíticas y memoria permanecen; si lo vuelve a habilitar más tarde, los datos históricos siguen ahí.

Use `Disabled` cuando:
- Quiere sacar un agente de la rotación sin perderlo.
- Un agente se está comportando mal y necesita detenerlo inmediatamente mientras investiga.
- Está rotando agentes estacionalmente dentro y fuera (p. ej., un agente que opera solo en fechas festivas).

### Dry Run - predeterminado para nuevos agentes

El agente se ejecuta de extremo a extremo: procesa disparadores, llama al LLM, selecciona llamadas a herramientas, calcula justificaciones y confianza, pero **no se realiza ninguna acción real**. Cada ejecución se registra con la insignia **Dry Run** en [Run History](#run-history).

Use `Dry Run` cuando:
- Un nuevo agente acaba de salir de la caja. Todos los templates iniciales empiezan en dry-run.
- Ha editado el prompt o cambiado el conjunto de disparadores y quiere ver cómo se comporta el cambio antes de comprometerlo.
- Está realizando una [ejecución de prueba / reejecución](#test-runs-replays) (las reejecuciones fuerzan dry-run independientemente del estado del agente).

La plataforma cobra tokens por las ejecuciones dry-run: la llamada al LLM aún ocurre, solo se omiten los efectos secundarios. Los topes de presupuesto se aplican también a dry-run. Vea [Budgets Overview](#budgets-overview).

### Enabled

El agente realiza acciones reales. Las llamadas a herramientas se ejecutan, o se encolan para [approval] si la acción está restringida.

Use `Enabled` después de que la salida en dry-run parezca correcta.

### Cambio de estado

Puede alternar entre cualquiera de los estados en el formulario de edición. Cambiar de Dry Run a Enabled no vuelve a ejecutar retroactivamente las acciones de dry-run: esas quedan como historial de dry-run. Los nuevos disparadores a partir de ese momento se ejecutan en vivo.

Cambiar de Enabled a Disabled a mitad de ejecución **no** aborta una ejecución en curso. El disparador que se está ejecutando actualmente termina (con lo que ya haya iniciado); el siguiente disparador se descarta porque el agente ahora está Disabled.

### Estado durante problemas de facturación

Si la facturación de su tenant se vuelve inválida, todos los agentes quedan efectivamente en pausa independientemente del estado guardado: los disparadores se descartan con `BILLING_INVALID` hasta que se restablezca la facturación. El campo de estado guardado no se modifica; el despachador simplemente se niega a ejecutar. Vea [Plans and Eligibility](#plans-and-eligibility).