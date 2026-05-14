D2L Brightspace expone el Registro dinámico a través de la interfaz de administración de LTI Advantage. Necesitará acceso de administrador.

#### Abra la pantalla de registro

1. Inicie sesión en su instancia de Brightspace como administrador.
2. Navegue a **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Haga clic en **Register Tool**. (La URL directa es `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Pegue la URL

Verá un formulario de registro. El campo clave es **Endpoint de registro de inicio de la herramienta** (algunas versiones de Brightspace lo etiquetan como "Tool Initiation Registration URL").

Pegue la URL de registro de FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguelo aquí</a>) en ese campo. Deje los demás campos en blanco: FastComments los completa automáticamente durante el intercambio de registro.

Haga clic en **Registrar**.

#### Apruebe la herramienta

Brightspace abre una ventana emergente que se comunica con FastComments, intercambia claves y muestra una pantalla de confirmación. La ventana emergente se cierra sola cuando se completa el registro.

La nueva herramienta aparece en la lista de herramientas de LTI Advantage. Por defecto Brightspace marca las herramientas nuevas como **deshabilitado**: cambie el interruptor a **habilitado** para que sus cursos puedan usarla.

#### Agregar un despliegue

En Brightspace, las herramientas LTI necesitan un **despliegue** antes de poder usarse en los cursos:

1. Abra la herramienta FastComments recién registrada.
2. Haga clic en **Ver despliegues** > **Nuevo despliegue**.
3. Asigne un nombre al despliegue (p. ej. "FastComments - All Courses"), seleccione las unidades organizativas en las que debe estar disponible y guarde.

Tras el primer lanzamiento a través de este despliegue, FastComments fija el `deployment_id` en su registro de configuración: los lanzamientos posteriores desde un despliegue distinto bajo el mismo cliente serán rechazados a menos que vuelva a registrarlo.