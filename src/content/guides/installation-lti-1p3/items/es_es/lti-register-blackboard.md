Blackboard Learn SaaS y Ultra son compatibles con LTI 1.3 Dynamic Registration.

#### Abrir la pantalla de proveedores de herramientas

1. Inicie sesión en Blackboard como administrador del sistema.
2. Navegue a **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Haga clic en **Register LTI 1.3 / LTI Advantage Tool**.

Si solo ve "Register LTI 1.1 Provider", su versión de Blackboard aún no es compatible con LTI 1.3: actualice o contacte al soporte de Blackboard.

#### Pegar la URL

Pegue la URL de registro de FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obténgala aquí</a>) en el campo **Client ID** / **Registration URL** (la etiqueta de Blackboard varía según la versión). Envíe.

Blackboard realiza el protocolo de registro con FastComments y le muestra una pantalla de confirmación.

#### Aprobar y habilitar

Blackboard marca las herramientas recién registradas como **Approved but excluded** de forma predeterminada:

1. Busque la entrada de FastComments en la lista de proveedores de herramientas.
2. Abra el menú y elija **Edit**.
3. Establezca **Tool Status** en **Approved**.
4. En **Institution Policies**, revise qué datos del usuario se envían (nombre, correo electrónico, rol). Guarde.

La herramienta ya está disponible para los instructores cuando agregan contenido a los cursos.