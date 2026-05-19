**¿Usas Moodle?** También publicamos un plugin dedicado para Moodle de FastComments con una integración más estrecha que LTI 1.3 (ganchos de sincronización de calificaciones, informes de actividad más detallados, interfaz de configuración nativa de Moodle). Consulta la <a href="/guide-installation-moodle.html" target="_blank">guía de instalación del plugin de Moodle</a>. El flujo LTI 1.3 que se muestra a continuación es la opción adecuada si quieres un registro único que también cubra otros LMSes, o si el administrador de Moodle no instalará plugins de terceros.

Moodle 4.0+ admite LTI 1.3 Dynamic Registration a través del plugin External Tool.

#### Abre la pantalla de gestión de herramientas

1. Inicia sesión en Moodle como administrador del sitio.
2. Navega a **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Pega la URL

Verás una tarjeta etiquetada **Tool URL**. Pega la URL de registro de FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguela aquí</a>) en el campo de texto y haz clic en **Add LTI Advantage**.

Moodle abrirá una pantalla de registro que muestra la identidad de la herramienta y los permisos que solicita. Revisa y haz clic en **Activate** (o **Register**, según la versión de Moodle).

La ventana emergente se cierra cuando se completa el registro; la nueva herramienta FastComments aparece en la lista **Tools** con el estado **Active**.

#### Hazla disponible

Por defecto, Moodle añade nuevas herramientas a la lista "Course tools" pero no las muestra en el selector de actividades. Para exponer FastComments en todo el curso:

1. Haz clic en el icono de engranaje del mosaico de FastComments.
2. En **Tool configuration usage**, selecciona **Show in activity chooser and as a preconfigured tool**.
3. Save.

Los instructores ahora pueden añadir FastComments a cualquier curso mediante **Add an activity or resource** > **FastComments**.