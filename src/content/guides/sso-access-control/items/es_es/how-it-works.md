El control de acceso de FastComments funciona asignando `Pages` y `Users` a los grupos deseados.

Un grupo es simplemente un identificador de cadena, como `GREEN` o `abc-123`.

`Users` y `Pages` no están limitados a un solo grupo. Están limitados a `100` y `1000` grupos, respectivamente. 

#### Acceso a páginas no autorizadas

Si un usuario intenta acceder a una página a la que no tiene acceso, verá un mensaje de error como el siguiente:

<div class="screenshot white-bg">
    <div class="title">Ejemplo de fallo de autorización</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Ejemplo de fallo de autorización" />
</div>

El texto del mensaje se puede personalizar.

---