#### Sakai

Sakai es compatible con LTI 1.3 Dynamic Registration en las versiones que incluyen LTI Advantage. Desde el Espacio de Administración:

1. Inicia sesión como administrador de Sakai y abre el **Espacio de Administración**.
2. Elige **Herramientas externas** > **Instalar herramienta LTI 1.3**.
3. Pega la URL de registro de FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguelo aquí</a>) y envía.
4. Aprueba la herramienta cuando se complete el handshake.

La herramienta aparecerá entonces bajo **Herramientas externas** y puede añadirse a los sitios por sus mantenedores.

#### Schoology

Las instancias Enterprise de Schoology son compatibles con LTI 1.3, pero la disponibilidad del Registro Dinámico varía según la implementación. Consulta con tu gestor de cuenta de Schoology.

Si el Registro Dinámico no está disponible en tu instancia de Schoology, tendrás que configurar la integración manualmente usando estos endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Después de que Schoology te proporcione un Client ID y Deployment ID, contacta con el soporte de FastComments para registrar la configuración en tu tenant.

#### Other LTI 1.3 Platforms

Cualquier LMS que siga la especificación IMS LTI 1.3 Advantage debería funcionar con la misma URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguelo aquí</a>). Busca una opción etiquetada como "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" o similar.

Si tu plataforma solo admite la configuración manual de LTI 1.3, usa los cuatro endpoints listados en la sección de Schoology más arriba y contacta con soporte para finalizar.