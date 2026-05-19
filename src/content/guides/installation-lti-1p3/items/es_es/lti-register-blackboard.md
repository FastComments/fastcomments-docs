Blackboard Learn SaaS y Ultra son compatibles con el registro dinámico LTI 1.3.

#### Open the Tool Provider Screen

1. Inicia sesión en Blackboard como administrador del sistema.
2. Navega a **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Haz clic en **Register LTI 1.3 / LTI Advantage Tool**.

Si solo ves "Register LTI 1.1 Provider", tu versión de Blackboard aún no es compatible con LTI 1.3: actualiza o contacta con el soporte de Blackboard.

#### Paste the URL

Pega la URL de registro de FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguela aquí</a>) en el campo **Client ID** / **Registration URL** (la denominación de Blackboard varía según la versión). Envía.

Blackboard realiza el intercambio de registro con FastComments y te muestra una pantalla de confirmación.

#### Approve and Enable

Blackboard marca las herramientas recién registradas como **Approved but excluded** por defecto:

1. Busca la entrada de FastComments en la lista de proveedores de herramientas.
2. Abre el menú y elige **Edit**.
3. Ajusta **Tool Status** a **Approved**.
4. En **Institution Policies**, revisa qué datos de usuario se envían (nombre, correo electrónico, rol). Guardar.

La herramienta ya está disponible para los instructores cuando añaden contenido a los cursos.