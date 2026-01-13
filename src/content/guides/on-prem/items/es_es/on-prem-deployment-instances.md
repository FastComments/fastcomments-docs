### Componentes Requeridos

Para On-Prem, FastComments consiste únicamente en un servidor de aplicación y una base de datos. Hemos simplificado el despliegue de modo que la aplicación pueda atender todo el tráfico directamente sin añadir otros componentes.

El servidor de aplicación se proporciona en una imagen Docker y puede desplegarse con cualquier solución de gestión de contenedores.

La base de datos, MongoDB, puede ejecutarse por cuenta propia o alojarse con otro proveedor como AWS DocumentDB o MongoDB Atlas.

FastComments se prueba actualmente con MongoDB 7; sin embargo, nuestro objetivo es ser compatibles con DocumentDB para facilitar el despliegue.

### Tamaños de instancia

Verá que FastComments está bastante optimizado y no requiere máquinas grandes para la propia aplicación para mantener P99s bajos.

Todos los trabajos por lotes y cron usan streaming para limitar el uso total de memoria.

Las tablas siguientes para el servidor de aplicación y la base de datos pueden ayudar con el dimensionamiento.

### Instancias del servidor de aplicación


| Concurrent Users | Total Cluster CPUs | Total Cluster Memory |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

Por ejemplo, un único núcleo atendiendo alrededor de 100 hilos de comentarios por segundo normalmente nunca usa más de 250mb RSS.

### Instancias del servidor de base de datos

Dimensionar la base de datos depende del tamaño del conjunto de trabajo (working set), que es la cantidad de datos a los que se accede en un momento dado, así como de las solicitudes concurrentes.

FastComments es bastante amable con Mongo, en el sentido de que para las consultas calientes utiliza index hints, streaming cursors, y tiene límites de concurrencia en varias áreas para evitar sobrecargar los sistemas aguas abajo.

Lo siguiente es una guía general sobre tamaños de instancias de base de datos. **Tenga en cuenta que esto es __por instancia__, no los recursos totales en el clúster**.

| Concurrent Users | Comments Stored | CPUs Per Instance | Memory Per Instance |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

Las tablas anteriores son estimaciones conservadoras. Puede que los requisitos reales difieran según su configuración específica (tamaños de página, volumen de comentarios, etc.).