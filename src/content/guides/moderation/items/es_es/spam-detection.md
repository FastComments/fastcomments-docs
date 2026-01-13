---
Por defecto, FastComments incluye detección de spam entrenable.

A medida que moderas los comentarios y los marcas como **Spam**, o marcas comentarios que se han encontrado automáticamente como **Spam** como **Not Spam**, el sistema de detección de spam aprenderá de estas acciones para determinar con mayor precisión qué quieres que sea spam.

Los comentarios marcados como **Spam** no se aprobarán automáticamente, por lo que no se mostrarán hasta que se marquen explícitamente como **Not Spam**.

La detección de spam puede desactivarse desde la página de Configuración de Moderación de Comentarios.

### Diferentes detectores de spam

FastComments admite tres formas de detectar spam:

1. Un clasificador Naïve-Bayes tradicional que se entrena continuamente, y que se comparte entre todos los tenants de FastComments.com.
2. Un clasificador Naïve-Bayes tradicional que se entrena continuamente, y que está **aislado** a su tenant.
3. Uso de ChatGPT 4.

Todos tienen acceso a los clasificadores Naïve-Bayes compartidos y aislados.

La opción ChatGPT 4 es seleccionable en la página de Configuración de Moderación de Comentarios si está en Flex billing, ya que la facturación se realiza en función de los tokens utilizados.

### Factor de confianza

FastComments ajusta el filtro de spam para un usuario según cuánto se le confíe en el sitio en cuestión.

Por ejemplo, si los administradores han fijado muchos de sus comentarios, probablemente sea un usuario muy confiable. O, si ha sido miembro del sitio durante mucho tiempo y tiene muchos comentarios, su factor de confianza también puede ser alto.

### SSO

Los comentarios publicados por usuarios SSO pueden considerarse spam y se comprobarán como tal. La excepción es si el usuario SSO tiene el mismo correo electrónico que un usuario del tenant que posee uno o más de los siguientes permisos:

- Account Owner
- Super Admin
- Comment Moderator Admin

Los usuarios SSO con estos permisos no tendrán sus comentarios verificados por spam.

### Mensajes repetidos

FastComments detectará y evitará mensajes repetidos. También detectará mensajes repetidos que sean muy similares para ayudar a prevenir el spam. Esto no puede desactivarse ya que evita que nuestra plataforma sea utilizada para abuso. Si tiene un factor de confianza alto, esto se tiene en cuenta al aplicar la prevención de mensajes repetidos.

---