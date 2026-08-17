Un agente tiene uno de tres estados:

### Deshabilitado

El agente está apagado. No se procesan disparadores y el agente no aparece en la ruta de despacho. Su historial de ejecuciones, análisis y memoria permanecen; si lo vuelves a habilitar más tarde, los datos históricos siguen allí.

Use `Disabled` cuando:
- Quieres retirar un agente de la rotación sin perderlo.
- Un agente se está comportando de forma incorrecta y necesitas detenerlo inmediatamente mientras lo investigas.
- Estás rotando agentes estacionalmente (p. ej., un saludo solo para vacaciones).

### Dry Run - predeterminado para nuevos agentes

El agente se ejecuta de extremo a extremo: procesa disparadores, llama al LLM, elige llamadas a herramientas, calcula justificaciones y confianza, pero **no se realiza ninguna acción real**. Cada ejecución se registra con la insignia **Dry Run** en [Run History](#run-history).

Use `Dry Run` cuando:
- Un nuevo agente está recién creado. Cada plantilla inicial se ejecuta en modo dry‑run.
- Has editado el prompt o cambiado el conjunto de disparadores y quieres ver cómo afecta el cambio antes de comprometerlo.
- Estás ejecutando una [prueba / reproducción](#test-runs-replays) (las reproducciones forzan dry‑run sin importar el estado del agente).

La plataforma cobra tokens por ejecuciones dry‑run: la llamada al LLM sigue ocurriendo, solo se omiten los efectos secundarios. Los límites de presupuesto también se aplican a dry‑run. Ver [Budgets Overview](#budgets-overview).

### Habilitado

El agente realiza acciones reales. Las llamadas a herramientas se ejecutan o se ponen en cola para [aprobación](#approval-workflow) si la acción está restringida.

Use `Enabled` después de que la salida del dry‑run parezca correcta.

### Cambiar estado

Puedes alternar entre cualquiera de los dos estados en el formulario de edición. Cambiar de Dry Run a Enabled no vuelve a ejecutar retroactivamente las acciones de dry‑run; esas permanecen en el historial de dry‑run. Los nuevos disparadores a partir de ese momento se ejecutan en vivo.

Cambiar de Enabled a Disabled a mitad de una ejecución **no** aborta una ejecución en curso. El disparador que se está ejecutando finaliza (con lo que ya haya iniciado); el siguiente disparador se descarta porque el agente ahora está Deshabilitado.

### Estado durante problemas de facturación

Si la facturación de tu inquilino se vuelve inválida, todos los agentes se pausarán efectivamente sin importar el estado guardado: los disparadores se descartan con `BILLING_INVALID` hasta que se restablezca la facturación. El campo de estado guardado no se cambia; el despachador simplemente se niega a ejecutar. Ver [Plans and Eligibility](#plans-and-eligibility).