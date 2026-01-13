Probablemente no quieras definir el config en línea si estás pasando callbacks, etc. En su lugar, querrás definir
el config en un bloque `computed`, de lo contrario cada vez que se invoque tu callback, etc., todo el widget se volverá a renderizar.

[Mira el ejemplo del spinner para ver cómo hacerlo.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)