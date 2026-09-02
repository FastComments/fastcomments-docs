Es posible prohibir usuarios que utilizan ciertos proveedores de correo electrónico mediante comodines.

Por ejemplo, si descubres que todos los comentarios de **@bademail.com** son spam, puedes simplemente prohibir todo el proveedor de correo ingresando "*@bademail.com" en el campo de correo electrónico al añadir un usuario prohibido.

Observa el "*" antes del @ en el correo electrónico.

### Subdominios

Una prohibición de dominio también cubre cada subdominio de ese dominio. Prohibir `*@bademail.com` también prohíbe `someone@mail.bademail.com` y `someone@eu.mail.bademail.com`, por lo que no es necesario añadir una prohibición separada para cada subdominio.

Si solo deseas prohibir un subdominio específico, ingresa ese subdominio en su lugar, por ejemplo `*@mail.bademail.com`. Esa prohibición no afecta a `someone@bademail.com`.

### Prohibir un dominio desde un comentario

No tienes que escribir el patrón tú mismo. Cuando prohibes a un usuario desde un comentario en la página Moderar Comentarios, el cuadro de diálogo de prohibición tiene una casilla de verificación "Prohibir todos los usuarios @domain" que crea la misma prohibición `*@domain` para el dominio de correo del comentarista.

### Patrones compatibles

La única forma de comodín compatible es un único `*` en lugar de la parte completa del nombre, seguido de `@` y un dominio. Otras formas son rechazadas cuando intentas guardarlas:

- `*@*.bademail.com` no es necesario, porque `*@bademail.com` ya cubre los subdominios.
- `name*@bademail.com` y `*bademail.com` no son compatibles.

---