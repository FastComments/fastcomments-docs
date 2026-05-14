#### Sakai

Sakai admite el Registro dinámico de LTI 1.3 en las versiones con LTI Advantage. Desde el Área de administración:

1. Inicie sesión como administrador de Sakai y abra el **Área de administración**.
2. Elija **Herramientas externas** > **Instalar herramienta LTI 1.3**.
3. Pegue la URL de registro de FastComments y envíe.
4. Apruebe la herramienta cuando se complete el intercambio de claves.

La herramienta aparecerá entonces bajo **Herramientas externas** y los responsables de los sitios podrán añadirla.

#### Schoology

Las instancias Enterprise de Schoology admiten LTI 1.3, pero la disponibilidad del Registro dinámico varía según la implementación. Consulte con su gestor de cuenta de Schoology.

Si el Registro dinámico no está disponible en su instancia de Schoology, deberá configurar la integración manualmente usando estos endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Después de que Schoology le proporcione un Client ID y un Deployment ID, contacte con el soporte de FastComments para registrar la configuración en su tenant.

#### Other LTI 1.3 Platforms

Cualquier LMS que siga la especificación IMS LTI 1.3 Advantage debería funcionar con la misma URL de registro. Busque una opción etiquetada como "Registro dinámico", "Tool Registration URL", "Tool initiation registration endpoint" o similar.

Si su plataforma solo admite la configuración manual de LTI 1.3, utilice los cuatro endpoints listados en la sección de Schoology más arriba y contacte con el soporte para finalizar.