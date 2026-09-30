El borde alrededor del cuadro de comentarios está formado por el propio borde del área de texto más algunas líneas finas que el widget dibuja a su alrededor. Para cambiar su color o la redondez de las esquinas, establezca estas variables CSS en lugar de aplicar estilos directamente al `textarea`. Las variables restilizan cada parte del borde a la vez, por lo que las esquinas y los colores siempre coinciden.

| Variable | Qué cambia | Predeterminado |
|---|---|---|
| `--fc-input-border-color` | Color del borde | `#bfbfbf` |
| `--fc-input-border-color-focus` | Color del borde mientras el usuario escribe | `#555` |
| `--fc-input-border-radius` | Redondez de las esquinas | `11px` |
| `--fc-input-border-start-start-radius` | Redondez de la esquina superior izquierda cuadrada (superior derecha en idiomas de derecha a izquierda) | `0` |

Agregue el CSS al cuadro **Custom CSS** en la [página de personalización del widget](https://fastcomments.com/auth/my-account/customize-widget), o páselo con la opción `customCSS`. Solo necesita establecer las variables que desea cambiar.

También puede usar el asistente **Comment box border** justo debajo del cuadro Custom CSS, que genera este CSS por usted.

## Cambiar el color del borde

[inline-code-attrs-start title = 'Color del borde'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Cambiar el color del borde mientras escribe

[inline-code-attrs-start title = 'Color del borde mientras escribe'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Redondear las cuatro esquinas

Por defecto la esquina superior izquierda es cuadrada. Establezca ambas variables de radio para redondear las cuatro esquinas de la misma manera:

[inline-code-attrs-start title = 'Redondear las cuatro esquinas'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Esquinas cuadradas

[inline-code-attrs-start title = 'Esquinas cuadradas'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Coincidir con su marca

[inline-code-attrs-start title = 'Colores y esquinas de la marca'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Colores diferentes en modo oscuro

Cuando el widget está en modo oscuro tiene la clase `dark`, por lo que puede establecer valores diferentes para el modo oscuro:

[inline-code-attrs-start title = 'Colores del borde en modo oscuro'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## ¿Por qué no estilizar directamente el área de texto?

El widget dibuja parte del borde del cuadro de comentarios por sí mismo, alrededor del área de texto. Si establece `border-color` o `border-radius` solo en el `textarea`, esas líneas mantienen el estilo predeterminado y el borde se ve desalineado, por ejemplo una línea cuadrada que atraviesa una esquina redondeada. Las variables anteriores cambian ambos a la vez.