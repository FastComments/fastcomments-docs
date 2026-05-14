#### Navegar a la Configuración LTI 1.3

Inicie sesión en FastComments y vaya a <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">su página de Configuración LTI 1.3</a>.

Si su cuenta aún no tiene acceso LTI, verá "LTI no está habilitado para esta cuenta" - póngase en contacto con el soporte para habilitarlo en su plan.

#### Seleccione una plataforma (Opcional)

En **Generar una URL de registro dinámica**, use el desplegable **Plataforma** para indicar a FastComments a qué LMS se está conectando:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Otra plataforma LTI 1.3

También puede dejarlo en **Detección automática**. La plataforma se lee desde la openid-configuration de su LMS durante el registro; el desplegable solo establece la etiqueta de visualización para la configuración resultante.

#### Generar la URL

Haga clic en **Generar URL**. FastComments crea un token de registro de un solo uso y le muestra una URL que se ve así:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Cópiela. Esta URL:

- Es de **uso único** - una vez que su LMS la llame correctamente, el token se consume.
- Expira después de **30 minutos** si no se utiliza.
- Debe mantenerse privada - cualquiera que tenga la URL puede registrar una herramienta en su tenant dentro de esos 30 minutos.

#### Configuraciones existentes

Una vez que se complete un registro con éxito, la nueva configuración aparece en la tabla **Configuraciones existentes** en la misma página, con su Plataforma, Emisor, ID de Cliente y Estado. Puede eliminar configuraciones de esta tabla si alguna vez necesita anular el registro.