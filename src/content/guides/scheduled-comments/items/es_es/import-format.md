### Ejemplo de formato

El CSV de Comentarios Programados se ve asi:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Detalles del formato

El archivo CSV de Comentarios Programados soporta las siguientes columnas:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

Las siguientes columnas son **requeridas**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Necesitaras la columna Parent ID si deseas soportar respuestas anidadas automatizadas.

### Como funciona el formato

El formato de importacion funciona asi:

1. Defines una fila en el CSV para cada comentario que deseas publicar.
2. El comentario se publica despues del retraso especificado (horas + minutos + segundos).
   1. Para importaciones programadas manualmente, los retrasos son relativos a cuando presionas "play" en la interfaz, despues de que la importacion esta completa - **no cuando comienza la importacion**.
   2. Para importaciones programadas automaticamente, el retraso es desde el momento de la carga de la pagina.
3. Debes definir un ID. Puedes simplemente usar identificadores incrementales como 1, 2, 3, 4, 5.
4. Si deseas usar respuestas anidadas, simplemente establece el valor de la columna `Parent ID` al `ID` de otro comentario.
5. El campo `Comment` soporta la misma sintaxis que FastComments soporta en su widget de comentarios para dar estilo al texto y agregar imagenes.
6. El campo `Avatar`, si se usa, debe ser una imagen accesible publicamente. Sera copiada y servida desde nuestra CDN.
7. Puedes eliminar todos los comentarios despues de la reproduccion, o si la reproduccion se detiene.
8. La eliminacion ocurre en vivo, por lo que puedes reutilizar la misma importacion programada una y otra vez.

### Un ejemplo

[Un archivo CSV de ejemplo esta aqui](/csv/fastcomments-scheduled-comments-example.csv).
