#### Sakai

Sakai es compatible con el Registro Dinámico LTI 1.3 en versiones con LTI Advantage. Desde el Administration Workspace:

1. Inicia sesión como administrador de Sakai y abre el **Administration Workspace**.
2. Elige **External Tools** > **Install LTI 1.3 Tool**.
3. Pega la URL de registro de FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguela aquí</a>) y envía.
4. Aprueba la herramienta cuando el handshake se complete.

La herramienta aparecerá entonces en **External Tools** y podrá ser añadida a los sitios por sus mantenedores.

#### Schoology

Las instancias de Schoology Enterprise son compatibles con LTI 1.3, pero la disponibilidad del Registro Dinámico varía según el despliegue. Consulta con tu gestor de cuenta de Schoology.

Si el Registro Dinámico no está disponible en tu instancia de Schoology, deberás configurar la integración manualmente usando estos endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Después de que Schoology te proporcione un Client ID y un Deployment ID, contacta con el soporte de FastComments para registrar la configuración en tu tenant.

#### Otras Plataformas LTI 1.3

Cualquier LMS que siga la especificación IMS LTI 1.3 Advantage debería funcionar con la misma URL de registro (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">consíguela aquí</a>). Busca una configuración etiquetada como "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" o similar.

Si tu plataforma solo admite la configuración manual de LTI 1.3, usa los cuatro endpoints listados en la sección de Schoology anterior y contacta con el soporte para finalizar.