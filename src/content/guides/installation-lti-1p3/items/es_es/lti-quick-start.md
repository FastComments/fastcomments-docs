1. Inicia sesión en FastComments y ve a <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">tu página de configuración LTI 1.3</a>.
2. (Opcional) Elige la plataforma desde la que te conectas en el desplegable **Platform** - establece la etiqueta de visualización, pero Auto-detect funciona bien.
3. Haz clic en **Generate URL**. Aparecerá una **Registration URL** de un solo uso (válida durante 30 minutos).
4. En tu LMS, abre la pantalla de Registro Dinámico LTI 1.3 y pega la URL en el campo **Tool initiation registration endpoint** (o equivalente). Envía.
5. Tu LMS realiza una llamada de retorno a FastComments, intercambia claves y crea la integración. La ventana emergente se cierra automáticamente cuando termina.
6. De vuelta en FastComments, la nueva configuración aparece en la tabla **Existing Configurations**. La herramienta ya está disponible dentro de los cursos de tu LMS.