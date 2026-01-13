Analytics del sitio, o simplemente llamado `Analytics` en el panel de control, proporciona una visión general de cómo su comunidad está usando FastComments en todos sus dominios.

FastComments proporciona algunas características únicas que muchas otras plataformas no ofrecen, como informes **en vivo** de usuarios en línea en cada página, y ordenar páginas por el número de usuarios en línea. Para hacer esto, simplemente visite la [Página de Analytics](https://fastcomments.com/auth/my-account/analytics) y haga clic en `Ordenar por usuarios en línea` bajo `Páginas principales`.

Tanto las métricas de `Usuarios en línea` como de `Páginas principales` son en vivo y se informan sin demora.

`Páginas principales` por defecto ordenará por el número de comentarios en cada página.

Finalmente, se proporciona un desglose para métricas totales en su inquilino, por día, a lo largo del tiempo para:

- Cargas de página
  - Este es el número de veces que un usuario abrió una página que contiene uno o más widgets de FastComments. Si la página contiene múltiples widgets, este número se incrementará por el número de widgets en esa página. Si tiene una SPA, cada vez que la aplicación abre un nuevo hilo de comentarios, este número se incrementaría. Esto aplica también a la biblioteca React Native.
  - Esta métrica también se usa para propósitos de facturación en los planes Flex.
- Comentarios dejados
  - Esto incluye todos los comentarios, independientemente del estado de verificación o aprobación, o si son spam o no.
- Votos dejados
  - Esto es para el número de votos dejados. Solo contará votos verificados, a menos que el voto anónimo esté habilitado.
- Cuentas creadas
  - Esta métrica es para cuando se agrega un nuevo usuario SSO, o un comentarista comenta con FastComments por primera vez usando su sitio.

Estas métricas son casi en tiempo real, con un retraso de hasta un minuto.
