D2L Brightspace expone el Registro Dinámico a través de la interfaz de administración LTI Advantage. Necesitará acceso de administrador.

#### Abrir la pantalla de registro

1. Inicie sesión en su instancia de Brightspace como administrador.
2. Navegue a **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Haga clic en **Register Tool**. (La URL directa es `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Pegar la URL

Verá un formulario de registro. El campo clave es **Tool initiation registration endpoint** (algunas versiones de Brightspace lo etiquetan como "Tool Initiation Registration URL").

Pegue la URL de registro de FastComments en ese campo. Deje los demás campos en blanco - FastComments los completará automáticamente durante el intercambio de registro.

Haga clic en **Register**.

#### Aprobar la herramienta

Brightspace abre una ventana emergente que se comunica con FastComments, intercambia claves y muestra una pantalla de confirmación. La ventana emergente se cierra automáticamente cuando finaliza el registro.

La nueva herramienta aparece en su lista de herramientas LTI Advantage. Por defecto Brightspace marca las nuevas herramientas como **disabled** - active el interruptor a **enabled** para que sus cursos puedan usarla.

#### Agregar un deployment

En Brightspace, las herramientas LTI necesitan un **deployment** antes de que puedan usarse en los cursos:

1. Abra la herramienta FastComments recién registrada.
2. Haga clic en **View Deployments** > **New Deployment**.
3. Dé al deployment un nombre (p. ej. "FastComments - All Courses"), seleccione las unidades organizativas en las que debería estar disponible y guarde.

Después del primer lanzamiento a través de este deployment, FastComments fija el `deployment_id` en su registro de configuración - los lanzamientos posteriores desde un deployment diferente bajo el mismo cliente serán rechazados a menos que vuelva a registrarse.