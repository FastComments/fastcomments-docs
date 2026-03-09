---
#### Abrir Claves de Desarrollador en Canvas

Inicie sesión en Canvas como administrador. Vaya a **Admin** (en la barra lateral izquierda) > seleccione su cuenta > **Developer Keys**.

#### Crear una clave de desarrollador LTI

Haga clic en **+ Developer Key** y seleccione **LTI Key**.

En el formulario de configuración:

1. En el campo **Redirect URIs** (lado izquierdo), pegue la **Launch URL** de la página de configuración de FastComments.
2. A la derecha, establezca **Method** en **Enter URL**.
3. Pegue la **Configuration URL** que copió de FastComments en el campo **JSON URL**.
4. Canvas cargará la configuración LTI automáticamente.
5. Asigne un nombre a la clave (p. ej., "FastComments").
6. Haga clic en **Save**.

#### Habilitar la clave de desarrollador

Tras guardar, la nueva clave aparecerá en la tabla Developer Keys con su **State** establecida en **OFF**. Haga clic en el interruptor para ponerla en **ON**. Canvas puede pedirle confirmación — haga clic en **Allow** para habilitar la clave.

#### Copiar el Client ID

La tabla Developer Keys muestra un **Client ID** numérico en la columna Details (p. ej. `17000000000042`). Copie este número - lo necesitará en el siguiente paso.

---