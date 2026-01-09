### Posicionamiento basado en porcentajes

Image Chat utiliza coordenadas basadas en porcentajes en lugar de coordenadas en píxeles para posicionar los marcadores de chat en las imágenes. Cuando un usuario hace clic en una imagen, el widget convierte las coordenadas en píxeles del clic en porcentajes respecto al ancho y alto de la imagen. Este enfoque garantiza que los marcadores permanezcan en la ubicación correcta independientemente de cómo se muestre la imagen.

Por ejemplo, si un usuario hace clic a 250 píxeles desde el borde izquierdo de una imagen de 1000px de ancho, el widget lo almacena como 25% desde la izquierda. Cuando otro usuario ve la misma imagen a 500px de ancho en un dispositivo móvil, el marcador aparece a 125 píxeles desde la izquierda, que sigue siendo el 25% del ancho.

### Beneficios para diseños responsivos

Este sistema de porcentajes hace que Image Chat funcione sin problemas en todos los tamaños de dispositivo y orientaciones. Sus imágenes pueden mostrarse en tamaños diferentes según el ancho de pantalla, las reglas de CSS o las restricciones del contenedor, pero los marcadores siempre se alinean correctamente con las ubicaciones previstas.

Los usuarios en ordenadores de sobremesa con monitores grandes ven los marcadores en las mismas posiciones relativas que los usuarios en smartphones con pantallas pequeñas. Los marcadores se escalan proporcionalmente con la propia imagen.

### Escalado de imagen y relación de aspecto

Siempre que su imagen mantenga su relación de aspecto al escalar (que es el comportamiento predeterminado del navegador), los marcadores basados en porcentajes permanecerán perfectamente alineados. El widget asume que las imágenes se escalan proporcionalmente y calcula las posiciones basándose en esta suposición.

Si aplica CSS que distorsione la relación de aspecto de la imagen (por ejemplo usando `object-fit: cover` con dimensiones específicas), las posiciones de los marcadores pueden no alinearse correctamente. Para obtener mejores resultados, permita que las imágenes se escalen de forma natural o use `object-fit: contain` para mantener la relación de aspecto.

### Tamaño del cuadrado de chat

El tamaño visual de los marcadores de chat también se basa en porcentajes. La opción de configuración `chatSquarePercentage` tiene por defecto 5%, lo que significa que cada cuadrado ocupa el 5% del ancho de la imagen. Esto asegura un peso visual consistente entre distintos tamaños de imagen.

En una imagen de 1000px de ancho con la configuración predeterminada del 5%, los marcadores tienen 50px de lado. En una imagen de 500px de ancho, los mismos marcadores tienen 25px de lado. Permanecen proporcionales al tamaño de la imagen.

### Comportamiento en móviles

En pantallas de menos de 768px de ancho, Image Chat cambia a un diseño optimizado para móviles. Las ventanas de chat se abren a pantalla completa en vez de flotar junto al marcador. Esto ofrece mejor usabilidad en pantallas pequeñas donde las ventanas flotantes oscurecerían demasiado la imagen.

Los propios marcadores permanecen visibles y clicables en sus posiciones basadas en porcentajes. Los usuarios aún pueden ver dónde están ubicadas todas las discusiones y tocar los marcadores para abrir la interfaz de chat a pantalla completa.

### Carga dinámica de imágenes

El sistema basado en porcentajes funciona correctamente incluso cuando las imágenes se cargan de forma asíncrona o cambian de tamaño después de cargar la página. El widget supervisa el elemento de la imagen y recalcula las posiciones de los marcadores cada vez que cambian las dimensiones de la imagen.

Si está usando carga diferida (lazy-loading) de imágenes o implementando imágenes responsivas con diferentes tamaños en distintos puntos de ruptura, los marcadores se ajustan automáticamente cuando cambia el tamaño de la imagen.

### Consistencia entre dispositivos

Porque las coordenadas se almacenan como porcentajes en la base de datos, una discusión creada en un equipo de escritorio aparece en la misma ubicación relativa exacta cuando se ve en una tableta o teléfono. Los usuarios pueden colaborar entre dispositivos sin inconsistencias de posicionamiento.

Esto funciona en ambas direcciones. Una discusión creada al tocar un punto específico en un dispositivo móvil aparece en la misma posición relativa cuando se visualiza en un monitor grande de escritorio.

### Consideraciones sobre el viewport

El widget calcula los porcentajes en relación con el propio elemento de la imagen, no con el viewport. Esto significa que desplazar la página o cambiar el tamaño de la ventana del navegador no afecta las posiciones de los marcadores. Los marcadores permanecen anclados a sus ubicaciones en la imagen independientemente de los cambios en el viewport.

### Preparación para el futuro

El enfoque basado en porcentajes hace que las discusiones sobre sus imágenes sean resistentes a cambios en el diseño, la disposición o el ecosistema de dispositivos. A medida que surjan nuevos tamaños de pantalla y dispositivos, las discusiones existentes seguirán mostrándose correctamente sin requerir actualizaciones o migraciones.

---